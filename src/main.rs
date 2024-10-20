use aws_sdk_s3::Client as S3Client;
use aws_config::meta::region::RegionProviderChain;
use aws_types::region::Region;
use aws_config::BehaviorVersion;
use tokio;

async fn get_public_buckets(s3_client: &S3Client) -> Vec<String> {
    let resp = s3_client
        .list_buckets()
        .send()
        .await
        .expect("Failed to list buckets");

    let mut public_buckets = Vec::new();
    for bucket in resp.buckets() {
        let bucket_name = bucket.name().unwrap_or_default().to_string();
        if is_bucket_public(s3_client, &bucket_name).await {
            public_buckets.push(bucket_name);
        }
    }

    public_buckets
}

async fn is_bucket_public(s3_client: &S3Client, bucket_name: &str) -> bool {
    let acl = s3_client
        .get_bucket_acl()
        .bucket(bucket_name)
        .send()
        .await
        .expect("Failed to get bucket ACL");

    for grant in acl.grants() {
        if let Some(grantee) = grant.grantee() {
            if grantee.uri().unwrap_or_default().contains("AllUsers") {
                return true; // Bucket is public
            }
        }
    }

    false
}

async fn check_s3_public_buckets(s3_client: &S3Client) {
    let public_buckets = get_public_buckets(&s3_client).await;
    if public_buckets.is_empty() {
        println!("No publicly accessible buckets found.");
    } else {
        println!("Publicly accessible buckets found:");
        for bucket in public_buckets {
            println!("- {}", bucket);
        }
    }
}

#[tokio::main]
async fn main() {
    // RegionProviderChain helps set the region. Adjust it based on your AWS region.
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"))
        .or_default_provider();  // No `.await` here

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;  // Call the function `latest()`
    let s3_client = S3Client::new(&config);

    println!("Scanning for publicly accessible S3 buckets...");
    check_s3_public_buckets(&s3_client).await;
}
