#!/bin/bash
set -euxo pipefail
set -o errexit
cd "$(dirname "$0")"

. setenv

aws --region us-east-1 cloudformation deploy \
    --template ../aws/templates/acm-certificate.yaml \
    --stack-name ${CFN_ACM_STACK_NAME} \
    --no-fail-on-empty-changeset \
    --parameter-overrides \
        DomainName=${CFN_DOMAIN_NAME} \
        SubDomain=${CFN_SUB_DOMAIN} \
        HostedZoneId=${CFN_HOSTEDZONEID} \
        S3BucketName=${APP_RESOURCE_BUCKET_NAME}

ACM_ARN=`aws --region us-east-1 cloudformation describe-stacks --stack-name ${CFN_ACM_STACK_NAME} | jq -r '.Stacks[] | .Outputs[] | select(.OutputKey == "CertificateArn") | .OutputValue '`

aws --region ap-northeast-1 cloudformation package \
    --template-file ../aws/templates/main.yaml \
    --s3-bucket ${CFN_TEMP_BUCKET_NAME} \
    --output-template-file ../deploywork/packaged.template

aws --region ap-northeast-1 cloudformation deploy \
    --stack-name ${CFN_MAIN_STACK_NAME} \
    --template-file ../deploywork/packaged.template \
    --capabilities CAPABILITY_NAMED_IAM CAPABILITY_AUTO_EXPAND \
    --no-fail-on-empty-changeset \
    --parameter-overrides \
        DomainName=${CFN_DOMAIN_NAME} \
        SubDomain=${CFN_SUB_DOMAIN} \
        HostedZoneId=${CFN_HOSTEDZONEID} \
        S3BucketName=${APP_RESOURCE_BUCKET_NAME} \
        CertificateArn=${ACM_ARN}

# build wasm module and copy interface file to vue project
../wasm/build.sh

# build vue project
cd ../vue/cubetrain && docker run -it --rm -v "$(pwd):/vue/cubetrain" node:17.8 /vue/cubetrain/build.sh && cd ../../scripts

# copy resource files to S3 bucket
aws s3 cp ../vue/cubetrain/dist/ s3://${APP_RESOURCE_BUCKET_NAME}/ --recursive
aws s3 cp ../wasm/pkg/ s3://${APP_RESOURCE_BUCKET_NAME}/wasm/ --recursive
aws s3 cp ../wasm/pkg/package_bg.wasm s3://${APP_RESOURCE_BUCKET_NAME}/wasm/package_bg.wasm --content-type "application/wasm"

# clear cache
CLOUDFRONT_DISTID=`aws cloudformation describe-stacks --stack-name ${CFN_MAIN_STACK_NAME} | jq -r '.Stacks[] | .Outputs[] | select(.OutputKey == "CFDistributionId") | .OutputValue '`
aws cloudfront create-invalidation --distribution-id ${CLOUDFRONT_DISTID} --paths "/*"