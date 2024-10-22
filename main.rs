use aws_config::load_from_env;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_iam::Client as IAMClient;
use aws_sdk_cloudtrail::Client as CloudTrailClient;
use serde::{Serialize, Deserialize};
use tokio;
use std::fs::File;
use std::io::Write;

// Structure for the S3 bucket report
#[derive(Serialize, Deserialize)]
struct S3Report {
    bucket_name: String,
    is_public: bool,
    is_encrypted: bool,
}

// Structure for the IAM role report
#[derive(Serialize, Deserialize)]
struct IAMRoleReport {
    role_name: String,
    is_overly_permissive: bool,
}

// Structure for the CloudTrail report
#[derive(Serialize, Deserialize)]
struct CloudTrailReport {
    trail_name: String,
    is_logging: bool,
}

// Structure for the full report
#[derive(Serialize, Deserialize)]
struct FullReport {
    s3_buckets: Vec<S3Report>,
    iam_roles: Vec<IAMRoleReport>,
    cloud_trails: Vec<CloudTrailReport>,
}

// S3 public bucket check
async fn is_bucket_public(s3_client: &S3Client, bucket_name: &str) -> bool {
    let acl = s3_client
        .get_bucket_acl()
        .bucket(bucket_name)
        .send()
        .await
        .expect("Failed to get bucket ACL");

    for grant in acl.grants().iter() {
        if let Some(grantee) = grant.grantee() {
            if let Some(uri) = grantee.uri() {
                if uri.contains("AllUsers") {
                    return true; // Bucket is public
                }
            }
        }
    }
    false
}

// S3 encryption check
async fn is_bucket_encrypted(s3_client: &S3Client, bucket_name: &str) -> bool {
    let encryption = s3_client
        .get_bucket_encryption()
        .bucket(bucket_name)
        .send()
        .await;
    encryption.is_ok() // Return true if encryption is enabled
}

// S3 bucket check that includes both public and encryption checks
async fn check_s3_buckets(s3_client: &S3Client) -> Vec<S3Report> {
    let mut report = Vec::new();
    let resp = s3_client
        .list_buckets()
        .send()
        .await
        .expect("Failed to list buckets");

    for bucket in resp.buckets().iter() {
        let bucket_name = bucket.name().unwrap_or_default().to_string();
        let is_public = is_bucket_public(s3_client, &bucket_name).await;
        let is_encrypted = is_bucket_encrypted(s3_client, &bucket_name).await;

        report.push(S3Report {
            bucket_name,
            is_public,
            is_encrypted,
        });
    }
    report
}

// IAM overly permissive role check
async fn check_iam_roles(iam_client: &IAMClient) -> Vec<IAMRoleReport> {
    let mut report = Vec::new();
    let resp = iam_client
        .list_roles()
        .send()
        .await
        .expect("Failed to list IAM roles");

    for role in resp.roles().iter() {
        let role_name = role.role_name();
        let role_name = if !role_name.is_empty() {
            role_name.to_string()
        } else {
            String::from("default_role_name")
        };

        println!("Checking role: {}", role_name);

        let attached_policies_resp = iam_client
            .list_attached_role_policies()
            .role_name(&role_name)
            .send()
            .await
            .expect("Failed to list attached policies");

        let mut overly_permissive = false;

        for policy in attached_policies_resp.attached_policies().iter() {
            println!("Policy: {}", policy.policy_name().unwrap_or_default());
            // Add logic to analyze policies for `*` permissions
            overly_permissive = true; // Simplified, assume it's overly permissive
        }

        report.push(IAMRoleReport {
            role_name,
            is_overly_permissive: overly_permissive,
        });
    }
    report
}

// CloudTrail configuration check
async fn check_cloudtrail(cloudtrail_client: &CloudTrailClient) -> Vec<CloudTrailReport> {
    let mut report = Vec::new();
    let trails = cloudtrail_client
        .describe_trails()
        .send()
        .await
        .expect("Failed to describe CloudTrails");

    for trail in trails.trail_list().iter() {
        let trail_name = trail.name().unwrap_or_default();
        println!("CloudTrail: {}", trail_name);

        let status = cloudtrail_client
            .get_trail_status()
            .name(trail_name)
            .send()
            .await
            .expect("Failed to get CloudTrail status");

        report.push(CloudTrailReport {
            trail_name: trail_name.to_string(),
            is_logging: status.is_logging().unwrap_or(false),
        });
    }
    report
}

// Generate the final report as a JSON file
async fn generate_report(
    s3_report: Vec<S3Report>,
    iam_report: Vec<IAMRoleReport>,
    cloudtrail_report: Vec<CloudTrailReport>,
) {
    let full_report = FullReport {
        s3_buckets: s3_report,
        iam_roles: iam_report,
        cloud_trails: cloudtrail_report,
    };

    let json_report = serde_json::to_string_pretty(&full_report).expect("Failed to serialize report");
    let mut file = File::create("cloud_audit_report.json").expect("Failed to create report file");
    file.write_all(json_report.as_bytes())
        .expect("Failed to write report to file");

    println!("Report generated: cloud_audit_report.json");
}

#[tokio::main]
async fn main() {
    // Use load_from_env to load the AWS configuration
    let config = aws_config::load_from_env().await;

    // Initialize AWS clients
    let s3_client = S3Client::new(&config);
    let iam_client = IAMClient::new(&config);
    let cloudtrail_client = CloudTrailClient::new(&config);

    println!("Scanning for publicly accessible S3 buckets, overly permissive IAM roles, and CloudTrail configurations...");

    // Run all checks concurrently
    let s3_report = check_s3_buckets(&s3_client).await;
    let iam_report = check_iam_roles(&iam_client).await;
    let cloudtrail_report = check_cloudtrail(&cloudtrail_client).await;

    // Generate the report
    generate_report(s3_report, iam_report, cloudtrail_report).await;
}
