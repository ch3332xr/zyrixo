# ![ZYRIXO](ZYRIXO.png)
#### CloudSec Misconfiguration Scanner 4 AWS*

### 🚀 Overview

*ZYRIXO* is a powerful tool designed to identify potential misconfigurations in your AWS cloud resources. By leveraging best practices and security guidelines, this scanner helps you maintain a secure cloud environment, ensuring your assets are protected against Security breaches/Vulnerabilities.

![alt text](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

### 📦 Features

- **Comprehensive Scanning**: Analyze various AWS services for misconfigurations.
 - ***NOTE***: *Currently supporting  AWS only.*
### 🔧 Installation

### Prerequisites

###### LOCAL

- RUST Version 1.82.0
- CARGO (comes with Rust)

##### DOCKER
- Docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

### *Run Locally*

1. *CLONE*
2. *CHDIR*
3. *COMPILE+BUILD*
4. *RUN*
   ```bash
   git clone https://github.com/ch3332xr/zyrixo.git
   cd zyrixo
   cargo build
   cargo run
### *Docker*
1. *CLONE*
2. *CHDIR*
3. *RUN*
   ```bash
   git clone https://github.com/ch3332xr/zyrixo.git
   cd zyrixo
   docker run -e AWS_ACCESS_KEY_ID=your_access_key \
           -e AWS_SECRET_ACCESS_KEY=your_secret_key \
           -e AWS_DEFAULT_REGION=your_region \
           zyrixo:latest
#### 🔧 Coming soon!
- *SUPPORT 4 OTHER AWS SERVICES*

#### 🤝 Contributing
 - Welcome
 - Fork/Branch/Changes/Push to branch
 
#### 📄 License
 - *TBD*

#### 📞 Contact
  - https://github.com/ch3332xr
  - 
 ### 📦 *BROUGHT TO U BY RAVSEC.IO*