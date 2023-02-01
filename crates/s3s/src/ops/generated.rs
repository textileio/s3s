#![allow(clippy::declare_interior_mutable_const)]
#![allow(clippy::borrow_interior_mutable_const)]

use crate::dto::*;
use crate::error::*;
use crate::header::names::*;
use crate::http;
use crate::path::S3Path;
use crate::s3_trait::S3;

use std::borrow::Cow;

impl http::TryIntoHeaderValue for ArchiveStatus {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for BucketCannedACL {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for ChecksumAlgorithm {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for ChecksumMode {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for MetadataDirective {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for ObjectAttributes {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for ObjectCannedACL {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for ObjectLockLegalHoldStatus {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for ObjectLockMode {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for ObjectOwnership {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for ReplicationStatus {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for RequestCharged {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for RequestPayer {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for ServerSideEncryption {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for StorageClass {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryIntoHeaderValue for TaggingDirective {
    type Error = http::InvalidHeaderValue;
    fn try_into_header_value(self) -> Result<http::HeaderValue, Self::Error> {
        match Cow::from(self) {
            Cow::Borrowed(s) => http::HeaderValue::try_from(s),
            Cow::Owned(s) => http::HeaderValue::try_from(s),
        }
    }
}

impl http::TryFromHeaderValue for ArchiveStatus {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for BucketCannedACL {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for ChecksumAlgorithm {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for ChecksumMode {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for MetadataDirective {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for ObjectAttributes {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for ObjectCannedACL {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for ObjectLockLegalHoldStatus {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for ObjectLockMode {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for ObjectOwnership {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for ReplicationStatus {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for RequestCharged {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for RequestPayer {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for ServerSideEncryption {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for StorageClass {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

impl http::TryFromHeaderValue for TaggingDirective {
    type Error = http::ParseHeaderError;
    fn try_from_header_value(val: &http::HeaderValue) -> Result<Self, Self::Error> {
        let val = val.to_str().map_err(|_| http::ParseHeaderError::Enum)?;
        Ok(Self::from(val.to_owned()))
    }
}

pub struct AbortMultipartUpload;

impl AbortMultipartUpload {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<AbortMultipartUploadInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let upload_id: MultipartUploadId = http::parse_query(req, "uploadId")?;

        Ok(AbortMultipartUploadInput {
            bucket,
            expected_bucket_owner,
            key,
            request_payer,
            upload_id,
        })
    }

    pub fn serialize_http(x: AbortMultipartUploadOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for AbortMultipartUpload {
    fn name(&self) -> &'static str {
        "AbortMultipartUpload"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.abort_multipart_upload(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct CompleteMultipartUpload;

impl CompleteMultipartUpload {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<CompleteMultipartUploadInput> {
        let (bucket, key) = http::unwrap_object(req);

        let checksum_crc32: Option<ChecksumCRC32> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_CRC32)?;

        let checksum_crc32c: Option<ChecksumCRC32C> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_CRC32C)?;

        let checksum_sha1: Option<ChecksumSHA1> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_SHA1)?;

        let checksum_sha256: Option<ChecksumSHA256> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_SHA256)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let multipart_upload: Option<CompletedMultipartUpload> = http::take_opt_xml_body(req)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let upload_id: MultipartUploadId = http::parse_query(req, "uploadId")?;

        Ok(CompleteMultipartUploadInput {
            bucket,
            checksum_crc32,
            checksum_crc32c,
            checksum_sha1,
            checksum_sha256,
            expected_bucket_owner,
            key,
            multipart_upload,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            upload_id,
        })
    }

    pub fn serialize_http(x: CompleteMultipartUploadOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED, x.bucket_key_enabled)?;
        http::add_opt_header(&mut res, X_AMZ_EXPIRATION, x.expiration)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for CompleteMultipartUpload {
    fn name(&self) -> &'static str {
        "CompleteMultipartUpload"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.complete_multipart_upload(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct CopyObject;

impl CopyObject {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<CopyObjectInput> {
        let (bucket, key) = http::unwrap_object(req);

        let acl: Option<ObjectCannedACL> = http::parse_opt_header(req, &X_AMZ_ACL)?;

        let bucket_key_enabled: BucketKeyEnabled =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED)?.unwrap_or(false);

        let cache_control: Option<CacheControl> = http::parse_opt_header(req, &CACHE_CONTROL)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_ALGORITHM)?;

        let content_disposition: Option<ContentDisposition> = http::parse_opt_header(req, &CONTENT_DISPOSITION)?;

        let content_encoding: Option<ContentEncoding> = http::parse_opt_header(req, &CONTENT_ENCODING)?;

        let content_language: Option<ContentLanguage> = http::parse_opt_header(req, &CONTENT_LANGUAGE)?;

        let content_type: Option<ContentType> = http::parse_opt_header(req, &CONTENT_TYPE)?;

        let copy_source: CopySource = http::parse_header(req, &X_AMZ_COPY_SOURCE)?;

        let copy_source_if_match: Option<CopySourceIfMatch> = http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_IF_MATCH)?;

        let copy_source_if_modified_since: Option<CopySourceIfModifiedSince> =
            http::parse_opt_header_timestamp(req, &X_AMZ_COPY_SOURCE_IF_MODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let copy_source_if_none_match: Option<CopySourceIfNoneMatch> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_IF_NONE_MATCH)?;

        let copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince> =
            http::parse_opt_header_timestamp(req, &X_AMZ_COPY_SOURCE_IF_UNMODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let copy_source_sse_customer_algorithm: Option<CopySourceSSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let copy_source_sse_customer_key: Option<CopySourceSSECustomerKey> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let copy_source_sse_customer_key_md5: Option<CopySourceSSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let expected_source_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_SOURCE_EXPECTED_BUCKET_OWNER)?;

        let expires: Option<Expires> = http::parse_opt_header_timestamp(req, &EXPIRES, TimestampFormat::HttpDate)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_opt_header(req, &X_AMZ_GRANT_FULL_CONTROL)?;

        let grant_read: Option<GrantRead> = http::parse_opt_header(req, &X_AMZ_GRANT_READ)?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_opt_header(req, &X_AMZ_GRANT_READ_ACP)?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE_ACP)?;

        let metadata: Option<Metadata> = http::parse_opt_metadata(req)?;

        let metadata_directive: Option<MetadataDirective> = http::parse_opt_header(req, &X_AMZ_METADATA_DIRECTIVE)?;

        let object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus> =
            http::parse_opt_header(req, &X_AMZ_OBJECT_LOCK_LEGAL_HOLD)?;

        let object_lock_mode: Option<ObjectLockMode> = http::parse_opt_header(req, &X_AMZ_OBJECT_LOCK_MODE)?;

        let object_lock_retain_until_date: Option<ObjectLockRetainUntilDate> =
            http::parse_opt_header_timestamp(req, &X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE, TimestampFormat::DateTime)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let ssekms_encryption_context: Option<SSEKMSEncryptionContext> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT)?;

        let ssekms_key_id: Option<SSEKMSKeyId> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID)?;

        let server_side_encryption: Option<ServerSideEncryption> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION)?;

        let storage_class: Option<StorageClass> = http::parse_opt_header(req, &X_AMZ_STORAGE_CLASS)?;

        let tagging: Option<TaggingHeader> = http::parse_opt_header(req, &X_AMZ_TAGGING)?;

        let tagging_directive: Option<TaggingDirective> = http::parse_opt_header(req, &X_AMZ_TAGGING_DIRECTIVE)?;

        let website_redirect_location: Option<WebsiteRedirectLocation> =
            http::parse_opt_header(req, &X_AMZ_WEBSITE_REDIRECT_LOCATION)?;

        Ok(CopyObjectInput {
            acl,
            bucket,
            bucket_key_enabled,
            cache_control,
            checksum_algorithm,
            content_disposition,
            content_encoding,
            content_language,
            content_type,
            copy_source,
            copy_source_if_match,
            copy_source_if_modified_since,
            copy_source_if_none_match,
            copy_source_if_unmodified_since,
            copy_source_sse_customer_algorithm,
            copy_source_sse_customer_key,
            copy_source_sse_customer_key_md5,
            expected_bucket_owner,
            expected_source_bucket_owner,
            expires,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write_acp,
            key,
            metadata,
            metadata_directive,
            object_lock_legal_hold_status,
            object_lock_mode,
            object_lock_retain_until_date,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            ssekms_encryption_context,
            ssekms_key_id,
            server_side_encryption,
            storage_class,
            tagging,
            tagging_directive,
            website_redirect_location,
        })
    }

    pub fn serialize_http(x: CopyObjectOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.copy_object_result {
            http::set_xml_body(&mut res, val)?;
        }
        http::add_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED, x.bucket_key_enabled)?;
        http::add_opt_header(&mut res, X_AMZ_COPY_SOURCE_VERSION_ID, x.copy_source_version_id)?;
        http::add_opt_header(&mut res, X_AMZ_EXPIRATION, x.expiration)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM, x.sse_customer_algorithm)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5, x.sse_customer_key_md5)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT, x.ssekms_encryption_context)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for CopyObject {
    fn name(&self) -> &'static str {
        "CopyObject"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.copy_object(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct CreateBucket;

impl CreateBucket {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<CreateBucketInput> {
        let bucket = http::unwrap_bucket(req);

        let acl: Option<BucketCannedACL> = http::parse_opt_header(req, &X_AMZ_ACL)?;

        let create_bucket_configuration: Option<CreateBucketConfiguration> = http::take_opt_xml_body(req)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_opt_header(req, &X_AMZ_GRANT_FULL_CONTROL)?;

        let grant_read: Option<GrantRead> = http::parse_opt_header(req, &X_AMZ_GRANT_READ)?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_opt_header(req, &X_AMZ_GRANT_READ_ACP)?;

        let grant_write: Option<GrantWrite> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE)?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE_ACP)?;

        let object_lock_enabled_for_bucket: ObjectLockEnabledForBucket =
            http::parse_opt_header(req, &X_AMZ_BUCKET_OBJECT_LOCK_ENABLED)?.unwrap_or(false);

        let object_ownership: Option<ObjectOwnership> = http::parse_opt_header(req, &X_AMZ_OBJECT_OWNERSHIP)?;

        Ok(CreateBucketInput {
            acl,
            bucket,
            create_bucket_configuration,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write,
            grant_write_acp,
            object_lock_enabled_for_bucket,
            object_ownership,
        })
    }

    pub fn serialize_http(x: CreateBucketOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, LOCATION, x.location)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for CreateBucket {
    fn name(&self) -> &'static str {
        "CreateBucket"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.create_bucket(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct CreateMultipartUpload;

impl CreateMultipartUpload {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<CreateMultipartUploadInput> {
        let (bucket, key) = http::unwrap_object(req);

        let acl: Option<ObjectCannedACL> = http::parse_opt_header(req, &X_AMZ_ACL)?;

        let bucket_key_enabled: BucketKeyEnabled =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED)?.unwrap_or(false);

        let cache_control: Option<CacheControl> = http::parse_opt_header(req, &CACHE_CONTROL)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_ALGORITHM)?;

        let content_disposition: Option<ContentDisposition> = http::parse_opt_header(req, &CONTENT_DISPOSITION)?;

        let content_encoding: Option<ContentEncoding> = http::parse_opt_header(req, &CONTENT_ENCODING)?;

        let content_language: Option<ContentLanguage> = http::parse_opt_header(req, &CONTENT_LANGUAGE)?;

        let content_type: Option<ContentType> = http::parse_opt_header(req, &CONTENT_TYPE)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let expires: Option<Expires> = http::parse_opt_header_timestamp(req, &EXPIRES, TimestampFormat::HttpDate)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_opt_header(req, &X_AMZ_GRANT_FULL_CONTROL)?;

        let grant_read: Option<GrantRead> = http::parse_opt_header(req, &X_AMZ_GRANT_READ)?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_opt_header(req, &X_AMZ_GRANT_READ_ACP)?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE_ACP)?;

        let metadata: Option<Metadata> = http::parse_opt_metadata(req)?;

        let object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus> =
            http::parse_opt_header(req, &X_AMZ_OBJECT_LOCK_LEGAL_HOLD)?;

        let object_lock_mode: Option<ObjectLockMode> = http::parse_opt_header(req, &X_AMZ_OBJECT_LOCK_MODE)?;

        let object_lock_retain_until_date: Option<ObjectLockRetainUntilDate> =
            http::parse_opt_header_timestamp(req, &X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE, TimestampFormat::DateTime)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let ssekms_encryption_context: Option<SSEKMSEncryptionContext> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT)?;

        let ssekms_key_id: Option<SSEKMSKeyId> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID)?;

        let server_side_encryption: Option<ServerSideEncryption> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION)?;

        let storage_class: Option<StorageClass> = http::parse_opt_header(req, &X_AMZ_STORAGE_CLASS)?;

        let tagging: Option<TaggingHeader> = http::parse_opt_header(req, &X_AMZ_TAGGING)?;

        let website_redirect_location: Option<WebsiteRedirectLocation> =
            http::parse_opt_header(req, &X_AMZ_WEBSITE_REDIRECT_LOCATION)?;

        Ok(CreateMultipartUploadInput {
            acl,
            bucket,
            bucket_key_enabled,
            cache_control,
            checksum_algorithm,
            content_disposition,
            content_encoding,
            content_language,
            content_type,
            expected_bucket_owner,
            expires,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write_acp,
            key,
            metadata,
            object_lock_legal_hold_status,
            object_lock_mode,
            object_lock_retain_until_date,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            ssekms_encryption_context,
            ssekms_key_id,
            server_side_encryption,
            storage_class,
            tagging,
            website_redirect_location,
        })
    }

    pub fn serialize_http(x: CreateMultipartUploadOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_opt_header_timestamp(&mut res, X_AMZ_ABORT_DATE, x.abort_date, TimestampFormat::HttpDate)?;
        http::add_opt_header(&mut res, X_AMZ_ABORT_RULE_ID, x.abort_rule_id)?;
        http::add_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED, x.bucket_key_enabled)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_ALGORITHM, x.checksum_algorithm)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM, x.sse_customer_algorithm)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5, x.sse_customer_key_md5)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT, x.ssekms_encryption_context)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for CreateMultipartUpload {
    fn name(&self) -> &'static str {
        "CreateMultipartUpload"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.create_multipart_upload(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteBucket;

impl DeleteBucket {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: DeleteBucketOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucket {
    fn name(&self) -> &'static str {
        "DeleteBucket"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteBucketAnalyticsConfiguration;

impl DeleteBucketAnalyticsConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketAnalyticsConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: AnalyticsId = http::parse_query(req, "id")?;

        Ok(DeleteBucketAnalyticsConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http(_: DeleteBucketAnalyticsConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketAnalyticsConfiguration {
    fn name(&self) -> &'static str {
        "DeleteBucketAnalyticsConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_analytics_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteBucketCors;

impl DeleteBucketCors {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketCorsInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketCorsInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: DeleteBucketCorsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketCors {
    fn name(&self) -> &'static str {
        "DeleteBucketCors"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_cors(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteBucketEncryption;

impl DeleteBucketEncryption {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketEncryptionInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketEncryptionInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: DeleteBucketEncryptionOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketEncryption {
    fn name(&self) -> &'static str {
        "DeleteBucketEncryption"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_encryption(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteBucketIntelligentTieringConfiguration;

impl DeleteBucketIntelligentTieringConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketIntelligentTieringConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let id: IntelligentTieringId = http::parse_query(req, "id")?;

        Ok(DeleteBucketIntelligentTieringConfigurationInput { bucket, id })
    }

    pub fn serialize_http(_: DeleteBucketIntelligentTieringConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketIntelligentTieringConfiguration {
    fn name(&self) -> &'static str {
        "DeleteBucketIntelligentTieringConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_intelligent_tiering_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteBucketInventoryConfiguration;

impl DeleteBucketInventoryConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketInventoryConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: InventoryId = http::parse_query(req, "id")?;

        Ok(DeleteBucketInventoryConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http(_: DeleteBucketInventoryConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketInventoryConfiguration {
    fn name(&self) -> &'static str {
        "DeleteBucketInventoryConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_inventory_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteBucketLifecycle;

impl DeleteBucketLifecycle {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketLifecycleInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketLifecycleInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: DeleteBucketLifecycleOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketLifecycle {
    fn name(&self) -> &'static str {
        "DeleteBucketLifecycle"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_lifecycle(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteBucketMetricsConfiguration;

impl DeleteBucketMetricsConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketMetricsConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: MetricsId = http::parse_query(req, "id")?;

        Ok(DeleteBucketMetricsConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http(_: DeleteBucketMetricsConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketMetricsConfiguration {
    fn name(&self) -> &'static str {
        "DeleteBucketMetricsConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_metrics_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteBucketOwnershipControls;

impl DeleteBucketOwnershipControls {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketOwnershipControlsInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketOwnershipControlsInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: DeleteBucketOwnershipControlsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketOwnershipControls {
    fn name(&self) -> &'static str {
        "DeleteBucketOwnershipControls"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_ownership_controls(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteBucketPolicy;

impl DeleteBucketPolicy {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketPolicyInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketPolicyInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: DeleteBucketPolicyOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketPolicy {
    fn name(&self) -> &'static str {
        "DeleteBucketPolicy"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_policy(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteBucketReplication;

impl DeleteBucketReplication {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketReplicationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketReplicationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: DeleteBucketReplicationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketReplication {
    fn name(&self) -> &'static str {
        "DeleteBucketReplication"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_replication(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteBucketTagging;

impl DeleteBucketTagging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketTaggingInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketTaggingInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: DeleteBucketTaggingOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketTagging {
    fn name(&self) -> &'static str {
        "DeleteBucketTagging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_tagging(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteBucketWebsite;

impl DeleteBucketWebsite {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteBucketWebsiteInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeleteBucketWebsiteInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: DeleteBucketWebsiteOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteBucketWebsite {
    fn name(&self) -> &'static str {
        "DeleteBucketWebsite"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_bucket_website(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteObject;

impl DeleteObject {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteObjectInput> {
        let (bucket, key) = http::unwrap_object(req);

        let bypass_governance_retention: BypassGovernanceRetention =
            http::parse_opt_header(req, &X_AMZ_BYPASS_GOVERNANCE_RETENTION)?.unwrap_or(false);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let mfa: Option<MFA> = http::parse_opt_header(req, &X_AMZ_MFA)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(DeleteObjectInput {
            bucket,
            bypass_governance_retention,
            expected_bucket_owner,
            key,
            mfa,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: DeleteObjectOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        http::add_header(&mut res, X_AMZ_DELETE_MARKER, x.delete_marker)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteObject {
    fn name(&self) -> &'static str {
        "DeleteObject"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_object(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteObjectTagging;

impl DeleteObjectTagging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteObjectTaggingInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(DeleteObjectTaggingInput {
            bucket,
            expected_bucket_owner,
            key,
            version_id,
        })
    }

    pub fn serialize_http(x: DeleteObjectTaggingOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteObjectTagging {
    fn name(&self) -> &'static str {
        "DeleteObjectTagging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_object_tagging(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeleteObjects;

impl DeleteObjects {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeleteObjectsInput> {
        let bucket = http::unwrap_bucket(req);

        let bypass_governance_retention: BypassGovernanceRetention =
            http::parse_opt_header(req, &X_AMZ_BYPASS_GOVERNANCE_RETENTION)?.unwrap_or(false);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let delete: Delete = http::take_xml_body(req)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let mfa: Option<MFA> = http::parse_opt_header(req, &X_AMZ_MFA)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        Ok(DeleteObjectsInput {
            bucket,
            bypass_governance_retention,
            checksum_algorithm,
            delete,
            expected_bucket_owner,
            mfa,
            request_payer,
        })
    }

    pub fn serialize_http(x: DeleteObjectsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeleteObjects {
    fn name(&self) -> &'static str {
        "DeleteObjects"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_objects(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct DeletePublicAccessBlock;

impl DeletePublicAccessBlock {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<DeletePublicAccessBlockInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(DeletePublicAccessBlockInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: DeletePublicAccessBlockOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        *res.status_mut() = http::StatusCode::NO_CONTENT;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for DeletePublicAccessBlock {
    fn name(&self) -> &'static str {
        "DeletePublicAccessBlock"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.delete_public_access_block(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketAccelerateConfiguration;

impl GetBucketAccelerateConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketAccelerateConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketAccelerateConfigurationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketAccelerateConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketAccelerateConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketAccelerateConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_accelerate_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketAcl;

impl GetBucketAcl {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketAclInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketAclInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketAclOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketAcl {
    fn name(&self) -> &'static str {
        "GetBucketAcl"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_acl(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketAnalyticsConfiguration;

impl GetBucketAnalyticsConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketAnalyticsConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: AnalyticsId = http::parse_query(req, "id")?;

        Ok(GetBucketAnalyticsConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http(x: GetBucketAnalyticsConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.analytics_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketAnalyticsConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketAnalyticsConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_analytics_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketCors;

impl GetBucketCors {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketCorsInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketCorsInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketCorsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketCors {
    fn name(&self) -> &'static str {
        "GetBucketCors"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_cors(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketEncryption;

impl GetBucketEncryption {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketEncryptionInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketEncryptionInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketEncryptionOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.server_side_encryption_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketEncryption {
    fn name(&self) -> &'static str {
        "GetBucketEncryption"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_encryption(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketIntelligentTieringConfiguration;

impl GetBucketIntelligentTieringConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketIntelligentTieringConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let id: IntelligentTieringId = http::parse_query(req, "id")?;

        Ok(GetBucketIntelligentTieringConfigurationInput { bucket, id })
    }

    pub fn serialize_http(x: GetBucketIntelligentTieringConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.intelligent_tiering_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketIntelligentTieringConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketIntelligentTieringConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_intelligent_tiering_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketInventoryConfiguration;

impl GetBucketInventoryConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketInventoryConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: InventoryId = http::parse_query(req, "id")?;

        Ok(GetBucketInventoryConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http(x: GetBucketInventoryConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.inventory_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketInventoryConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketInventoryConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_inventory_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketLifecycleConfiguration;

impl GetBucketLifecycleConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketLifecycleConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketLifecycleConfigurationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketLifecycleConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketLifecycleConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketLifecycleConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_lifecycle_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketLocation;

impl GetBucketLocation {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketLocationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketLocationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketLocationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketLocation {
    fn name(&self) -> &'static str {
        "GetBucketLocation"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_location(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketLogging;

impl GetBucketLogging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketLoggingInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketLoggingInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketLoggingOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketLogging {
    fn name(&self) -> &'static str {
        "GetBucketLogging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_logging(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketMetricsConfiguration;

impl GetBucketMetricsConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketMetricsConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: MetricsId = http::parse_query(req, "id")?;

        Ok(GetBucketMetricsConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http(x: GetBucketMetricsConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.metrics_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketMetricsConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketMetricsConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_metrics_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketNotificationConfiguration;

impl GetBucketNotificationConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketNotificationConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketNotificationConfigurationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketNotificationConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketNotificationConfiguration {
    fn name(&self) -> &'static str {
        "GetBucketNotificationConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_notification_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketOwnershipControls;

impl GetBucketOwnershipControls {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketOwnershipControlsInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketOwnershipControlsInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketOwnershipControlsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.ownership_controls {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketOwnershipControls {
    fn name(&self) -> &'static str {
        "GetBucketOwnershipControls"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_ownership_controls(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketPolicy;

impl GetBucketPolicy {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketPolicyInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketPolicyInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketPolicyOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(val) = x.policy {
            *res.body_mut() = http::Body::from(val);
        }
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketPolicy {
    fn name(&self) -> &'static str {
        "GetBucketPolicy"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_policy(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketPolicyStatus;

impl GetBucketPolicyStatus {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketPolicyStatusInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketPolicyStatusInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketPolicyStatusOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.policy_status {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketPolicyStatus {
    fn name(&self) -> &'static str {
        "GetBucketPolicyStatus"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_policy_status(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketReplication;

impl GetBucketReplication {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketReplicationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketReplicationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketReplicationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.replication_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketReplication {
    fn name(&self) -> &'static str {
        "GetBucketReplication"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_replication(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketRequestPayment;

impl GetBucketRequestPayment {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketRequestPaymentInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketRequestPaymentInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketRequestPaymentOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketRequestPayment {
    fn name(&self) -> &'static str {
        "GetBucketRequestPayment"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_request_payment(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketTagging;

impl GetBucketTagging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketTaggingInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketTaggingInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketTaggingOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketTagging {
    fn name(&self) -> &'static str {
        "GetBucketTagging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_tagging(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketVersioning;

impl GetBucketVersioning {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketVersioningInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketVersioningInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketVersioningOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketVersioning {
    fn name(&self) -> &'static str {
        "GetBucketVersioning"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_versioning(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetBucketWebsite;

impl GetBucketWebsite {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetBucketWebsiteInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetBucketWebsiteInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetBucketWebsiteOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetBucketWebsite {
    fn name(&self) -> &'static str {
        "GetBucketWebsite"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_bucket_website(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetObject;

impl GetObject {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectInput> {
        let (bucket, key) = http::unwrap_object(req);

        let checksum_mode: Option<ChecksumMode> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_MODE)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let if_match: Option<IfMatch> = http::parse_opt_header(req, &IF_MATCH)?;

        let if_modified_since: Option<IfModifiedSince> =
            http::parse_opt_header_timestamp(req, &IF_MODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let if_none_match: Option<IfNoneMatch> = http::parse_opt_header(req, &IF_NONE_MATCH)?;

        let if_unmodified_since: Option<IfUnmodifiedSince> =
            http::parse_opt_header_timestamp(req, &IF_UNMODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let part_number: PartNumber = http::parse_opt_query(req, "partNumber")?.unwrap_or(0);

        let range: Option<Range> = http::parse_opt_header(req, &RANGE)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let response_cache_control: Option<ResponseCacheControl> = http::parse_opt_query(req, "response-cache-control")?;

        let response_content_disposition: Option<ResponseContentDisposition> =
            http::parse_opt_query(req, "response-content-disposition")?;

        let response_content_encoding: Option<ResponseContentEncoding> = http::parse_opt_query(req, "response-content-encoding")?;

        let response_content_language: Option<ResponseContentLanguage> = http::parse_opt_query(req, "response-content-language")?;

        let response_content_type: Option<ResponseContentType> = http::parse_opt_query(req, "response-content-type")?;

        let response_expires: Option<ResponseExpires> =
            http::parse_opt_query_timestamp(req, "response-expires", TimestampFormat::HttpDate)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(GetObjectInput {
            bucket,
            checksum_mode,
            expected_bucket_owner,
            if_match,
            if_modified_since,
            if_none_match,
            if_unmodified_since,
            key,
            part_number,
            range,
            request_payer,
            response_cache_control,
            response_content_disposition,
            response_content_encoding,
            response_content_language,
            response_content_type,
            response_expires,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            version_id,
        })
    }

    pub fn serialize_http(x: GetObjectOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(val) = x.body {
            http::set_stream_body(&mut res, val);
        }
        http::add_opt_header(&mut res, ACCEPT_RANGES, x.accept_ranges)?;
        http::add_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED, x.bucket_key_enabled)?;
        http::add_opt_header(&mut res, CACHE_CONTROL, x.cache_control)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32, x.checksum_crc32)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32C, x.checksum_crc32c)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA1, x.checksum_sha1)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA256, x.checksum_sha256)?;
        http::add_opt_header(&mut res, CONTENT_DISPOSITION, x.content_disposition)?;
        http::add_opt_header(&mut res, CONTENT_ENCODING, x.content_encoding)?;
        http::add_opt_header(&mut res, CONTENT_LANGUAGE, x.content_language)?;
        http::add_header(&mut res, CONTENT_LENGTH, x.content_length)?;
        http::add_opt_header(&mut res, CONTENT_RANGE, x.content_range)?;
        http::add_opt_header(&mut res, CONTENT_TYPE, x.content_type)?;
        http::add_header(&mut res, X_AMZ_DELETE_MARKER, x.delete_marker)?;
        http::add_opt_header(&mut res, ETAG, x.e_tag)?;
        http::add_opt_header(&mut res, X_AMZ_EXPIRATION, x.expiration)?;
        http::add_opt_header_timestamp(&mut res, EXPIRES, x.expires, TimestampFormat::HttpDate)?;
        http::add_opt_header_timestamp(&mut res, LAST_MODIFIED, x.last_modified, TimestampFormat::HttpDate)?;
        http::add_opt_metadata(&mut res, x.metadata)?;
        http::add_header(&mut res, X_AMZ_MISSING_META, x.missing_meta)?;
        http::add_opt_header(&mut res, X_AMZ_OBJECT_LOCK_LEGAL_HOLD, x.object_lock_legal_hold_status)?;
        http::add_opt_header(&mut res, X_AMZ_OBJECT_LOCK_MODE, x.object_lock_mode)?;
        http::add_opt_header_timestamp(
            &mut res,
            X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE,
            x.object_lock_retain_until_date,
            TimestampFormat::DateTime,
        )?;
        http::add_header(&mut res, X_AMZ_MP_PARTS_COUNT, x.parts_count)?;
        http::add_opt_header(&mut res, X_AMZ_REPLICATION_STATUS, x.replication_status)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_RESTORE, x.restore)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM, x.sse_customer_algorithm)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5, x.sse_customer_key_md5)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        http::add_opt_header(&mut res, X_AMZ_STORAGE_CLASS, x.storage_class)?;
        http::add_header(&mut res, X_AMZ_TAGGING_COUNT, x.tag_count)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        http::add_opt_header(&mut res, X_AMZ_WEBSITE_REDIRECT_LOCATION, x.website_redirect_location)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObject {
    fn name(&self) -> &'static str {
        "GetObject"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetObjectAcl;

impl GetObjectAcl {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectAclInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(GetObjectAclInput {
            bucket,
            expected_bucket_owner,
            key,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: GetObjectAclOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectAcl {
    fn name(&self) -> &'static str {
        "GetObjectAcl"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_acl(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetObjectAttributes;

impl GetObjectAttributes {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectAttributesInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let max_parts: MaxParts = http::parse_opt_header(req, &X_AMZ_MAX_PARTS)?.unwrap_or(0);

        let object_attributes: ObjectAttributesList = http::parse_list_header(req, &X_AMZ_OBJECT_ATTRIBUTES)?;

        let part_number_marker: Option<PartNumberMarker> = http::parse_opt_header(req, &X_AMZ_PART_NUMBER_MARKER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(GetObjectAttributesInput {
            bucket,
            expected_bucket_owner,
            key,
            max_parts,
            object_attributes,
            part_number_marker,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            version_id,
        })
    }

    pub fn serialize_http(x: GetObjectAttributesOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_header(&mut res, X_AMZ_DELETE_MARKER, x.delete_marker)?;
        http::add_opt_header_timestamp(&mut res, LAST_MODIFIED, x.last_modified, TimestampFormat::HttpDate)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectAttributes {
    fn name(&self) -> &'static str {
        "GetObjectAttributes"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_attributes(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetObjectLegalHold;

impl GetObjectLegalHold {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectLegalHoldInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(GetObjectLegalHoldInput {
            bucket,
            expected_bucket_owner,
            key,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: GetObjectLegalHoldOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.legal_hold {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectLegalHold {
    fn name(&self) -> &'static str {
        "GetObjectLegalHold"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_legal_hold(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetObjectLockConfiguration;

impl GetObjectLockConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectLockConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetObjectLockConfigurationInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetObjectLockConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.object_lock_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectLockConfiguration {
    fn name(&self) -> &'static str {
        "GetObjectLockConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_lock_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetObjectRetention;

impl GetObjectRetention {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectRetentionInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(GetObjectRetentionInput {
            bucket,
            expected_bucket_owner,
            key,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: GetObjectRetentionOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.retention {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectRetention {
    fn name(&self) -> &'static str {
        "GetObjectRetention"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_retention(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetObjectTagging;

impl GetObjectTagging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectTaggingInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(GetObjectTaggingInput {
            bucket,
            expected_bucket_owner,
            key,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: GetObjectTaggingOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectTagging {
    fn name(&self) -> &'static str {
        "GetObjectTagging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_tagging(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetObjectTorrent;

impl GetObjectTorrent {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetObjectTorrentInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        Ok(GetObjectTorrentInput {
            bucket,
            expected_bucket_owner,
            key,
            request_payer,
        })
    }

    pub fn serialize_http(x: GetObjectTorrentOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(val) = x.body {
            http::set_stream_body(&mut res, val);
        }
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetObjectTorrent {
    fn name(&self) -> &'static str {
        "GetObjectTorrent"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_object_torrent(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct GetPublicAccessBlock;

impl GetPublicAccessBlock {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<GetPublicAccessBlockInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(GetPublicAccessBlockInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: GetPublicAccessBlockOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.public_access_block_configuration {
            http::set_xml_body(&mut res, val)?;
        }
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for GetPublicAccessBlock {
    fn name(&self) -> &'static str {
        "GetPublicAccessBlock"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.get_public_access_block(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct HeadBucket;

impl HeadBucket {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<HeadBucketInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(HeadBucketInput {
            bucket,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: HeadBucketOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for HeadBucket {
    fn name(&self) -> &'static str {
        "HeadBucket"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.head_bucket(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct HeadObject;

impl HeadObject {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<HeadObjectInput> {
        let (bucket, key) = http::unwrap_object(req);

        let checksum_mode: Option<ChecksumMode> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_MODE)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let if_match: Option<IfMatch> = http::parse_opt_header(req, &IF_MATCH)?;

        let if_modified_since: Option<IfModifiedSince> =
            http::parse_opt_header_timestamp(req, &IF_MODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let if_none_match: Option<IfNoneMatch> = http::parse_opt_header(req, &IF_NONE_MATCH)?;

        let if_unmodified_since: Option<IfUnmodifiedSince> =
            http::parse_opt_header_timestamp(req, &IF_UNMODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let part_number: PartNumber = http::parse_opt_query(req, "partNumber")?.unwrap_or(0);

        let range: Option<Range> = http::parse_opt_header(req, &RANGE)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(HeadObjectInput {
            bucket,
            checksum_mode,
            expected_bucket_owner,
            if_match,
            if_modified_since,
            if_none_match,
            if_unmodified_since,
            key,
            part_number,
            range,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            version_id,
        })
    }

    pub fn serialize_http(x: HeadObjectOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, ACCEPT_RANGES, x.accept_ranges)?;
        http::add_opt_header(&mut res, X_AMZ_ARCHIVE_STATUS, x.archive_status)?;
        http::add_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED, x.bucket_key_enabled)?;
        http::add_opt_header(&mut res, CACHE_CONTROL, x.cache_control)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32, x.checksum_crc32)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32C, x.checksum_crc32c)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA1, x.checksum_sha1)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA256, x.checksum_sha256)?;
        http::add_opt_header(&mut res, CONTENT_DISPOSITION, x.content_disposition)?;
        http::add_opt_header(&mut res, CONTENT_ENCODING, x.content_encoding)?;
        http::add_opt_header(&mut res, CONTENT_LANGUAGE, x.content_language)?;
        http::add_header(&mut res, CONTENT_LENGTH, x.content_length)?;
        http::add_opt_header(&mut res, CONTENT_TYPE, x.content_type)?;
        http::add_header(&mut res, X_AMZ_DELETE_MARKER, x.delete_marker)?;
        http::add_opt_header(&mut res, ETAG, x.e_tag)?;
        http::add_opt_header(&mut res, X_AMZ_EXPIRATION, x.expiration)?;
        http::add_opt_header_timestamp(&mut res, EXPIRES, x.expires, TimestampFormat::HttpDate)?;
        http::add_opt_header_timestamp(&mut res, LAST_MODIFIED, x.last_modified, TimestampFormat::HttpDate)?;
        http::add_opt_metadata(&mut res, x.metadata)?;
        http::add_header(&mut res, X_AMZ_MISSING_META, x.missing_meta)?;
        http::add_opt_header(&mut res, X_AMZ_OBJECT_LOCK_LEGAL_HOLD, x.object_lock_legal_hold_status)?;
        http::add_opt_header(&mut res, X_AMZ_OBJECT_LOCK_MODE, x.object_lock_mode)?;
        http::add_opt_header_timestamp(
            &mut res,
            X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE,
            x.object_lock_retain_until_date,
            TimestampFormat::DateTime,
        )?;
        http::add_header(&mut res, X_AMZ_MP_PARTS_COUNT, x.parts_count)?;
        http::add_opt_header(&mut res, X_AMZ_REPLICATION_STATUS, x.replication_status)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_RESTORE, x.restore)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM, x.sse_customer_algorithm)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5, x.sse_customer_key_md5)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        http::add_opt_header(&mut res, X_AMZ_STORAGE_CLASS, x.storage_class)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        http::add_opt_header(&mut res, X_AMZ_WEBSITE_REDIRECT_LOCATION, x.website_redirect_location)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for HeadObject {
    fn name(&self) -> &'static str {
        "HeadObject"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.head_object(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct ListBucketAnalyticsConfigurations;

impl ListBucketAnalyticsConfigurations {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListBucketAnalyticsConfigurationsInput> {
        let bucket = http::unwrap_bucket(req);

        let continuation_token: Option<Token> = http::parse_opt_query(req, "continuation-token")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(ListBucketAnalyticsConfigurationsInput {
            bucket,
            continuation_token,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: ListBucketAnalyticsConfigurationsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListBucketAnalyticsConfigurations {
    fn name(&self) -> &'static str {
        "ListBucketAnalyticsConfigurations"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_bucket_analytics_configurations(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct ListBucketIntelligentTieringConfigurations;

impl ListBucketIntelligentTieringConfigurations {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListBucketIntelligentTieringConfigurationsInput> {
        let bucket = http::unwrap_bucket(req);

        let continuation_token: Option<Token> = http::parse_opt_query(req, "continuation-token")?;

        Ok(ListBucketIntelligentTieringConfigurationsInput {
            bucket,
            continuation_token,
        })
    }

    pub fn serialize_http(x: ListBucketIntelligentTieringConfigurationsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListBucketIntelligentTieringConfigurations {
    fn name(&self) -> &'static str {
        "ListBucketIntelligentTieringConfigurations"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_bucket_intelligent_tiering_configurations(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct ListBucketInventoryConfigurations;

impl ListBucketInventoryConfigurations {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListBucketInventoryConfigurationsInput> {
        let bucket = http::unwrap_bucket(req);

        let continuation_token: Option<Token> = http::parse_opt_query(req, "continuation-token")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(ListBucketInventoryConfigurationsInput {
            bucket,
            continuation_token,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: ListBucketInventoryConfigurationsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListBucketInventoryConfigurations {
    fn name(&self) -> &'static str {
        "ListBucketInventoryConfigurations"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_bucket_inventory_configurations(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct ListBucketMetricsConfigurations;

impl ListBucketMetricsConfigurations {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListBucketMetricsConfigurationsInput> {
        let bucket = http::unwrap_bucket(req);

        let continuation_token: Option<Token> = http::parse_opt_query(req, "continuation-token")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(ListBucketMetricsConfigurationsInput {
            bucket,
            continuation_token,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(x: ListBucketMetricsConfigurationsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListBucketMetricsConfigurations {
    fn name(&self) -> &'static str {
        "ListBucketMetricsConfigurations"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_bucket_metrics_configurations(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct ListBuckets;

impl ListBuckets {
    pub fn deserialize_http(_: &mut http::Request) -> S3Result<ListBucketsInput> {
        Ok(ListBucketsInput {})
    }

    pub fn serialize_http(x: ListBucketsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListBuckets {
    fn name(&self) -> &'static str {
        "ListBuckets"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_buckets(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct ListMultipartUploads;

impl ListMultipartUploads {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListMultipartUploadsInput> {
        let bucket = http::unwrap_bucket(req);

        let delimiter: Option<Delimiter> = http::parse_opt_query(req, "delimiter")?;

        let encoding_type: Option<EncodingType> = http::parse_opt_query(req, "encoding-type")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let key_marker: Option<KeyMarker> = http::parse_opt_query(req, "key-marker")?;

        let max_uploads: MaxUploads = http::parse_opt_query(req, "max-uploads")?.unwrap_or(0);

        let prefix: Option<Prefix> = http::parse_opt_query(req, "prefix")?;

        let upload_id_marker: Option<UploadIdMarker> = http::parse_opt_query(req, "upload-id-marker")?;

        Ok(ListMultipartUploadsInput {
            bucket,
            delimiter,
            encoding_type,
            expected_bucket_owner,
            key_marker,
            max_uploads,
            prefix,
            upload_id_marker,
        })
    }

    pub fn serialize_http(x: ListMultipartUploadsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListMultipartUploads {
    fn name(&self) -> &'static str {
        "ListMultipartUploads"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_multipart_uploads(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct ListObjectVersions;

impl ListObjectVersions {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListObjectVersionsInput> {
        let bucket = http::unwrap_bucket(req);

        let delimiter: Option<Delimiter> = http::parse_opt_query(req, "delimiter")?;

        let encoding_type: Option<EncodingType> = http::parse_opt_query(req, "encoding-type")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let key_marker: Option<KeyMarker> = http::parse_opt_query(req, "key-marker")?;

        let max_keys: MaxKeys = http::parse_opt_query(req, "max-keys")?.unwrap_or(0);

        let prefix: Option<Prefix> = http::parse_opt_query(req, "prefix")?;

        let version_id_marker: Option<VersionIdMarker> = http::parse_opt_query(req, "version-id-marker")?;

        Ok(ListObjectVersionsInput {
            bucket,
            delimiter,
            encoding_type,
            expected_bucket_owner,
            key_marker,
            max_keys,
            prefix,
            version_id_marker,
        })
    }

    pub fn serialize_http(x: ListObjectVersionsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListObjectVersions {
    fn name(&self) -> &'static str {
        "ListObjectVersions"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_object_versions(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct ListObjects;

impl ListObjects {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListObjectsInput> {
        let bucket = http::unwrap_bucket(req);

        let delimiter: Option<Delimiter> = http::parse_opt_query(req, "delimiter")?;

        let encoding_type: Option<EncodingType> = http::parse_opt_query(req, "encoding-type")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let marker: Option<Marker> = http::parse_opt_query(req, "marker")?;

        let max_keys: MaxKeys = http::parse_opt_query(req, "max-keys")?.unwrap_or(0);

        let prefix: Option<Prefix> = http::parse_opt_query(req, "prefix")?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        Ok(ListObjectsInput {
            bucket,
            delimiter,
            encoding_type,
            expected_bucket_owner,
            marker,
            max_keys,
            prefix,
            request_payer,
        })
    }

    pub fn serialize_http(x: ListObjectsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListObjects {
    fn name(&self) -> &'static str {
        "ListObjects"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_objects(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct ListObjectsV2;

impl ListObjectsV2 {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListObjectsV2Input> {
        let bucket = http::unwrap_bucket(req);

        let continuation_token: Option<Token> = http::parse_opt_query(req, "continuation-token")?;

        let delimiter: Option<Delimiter> = http::parse_opt_query(req, "delimiter")?;

        let encoding_type: Option<EncodingType> = http::parse_opt_query(req, "encoding-type")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let fetch_owner: FetchOwner = http::parse_opt_query(req, "fetch-owner")?.unwrap_or(false);

        let max_keys: MaxKeys = http::parse_opt_query(req, "max-keys")?.unwrap_or(0);

        let prefix: Option<Prefix> = http::parse_opt_query(req, "prefix")?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let start_after: Option<StartAfter> = http::parse_opt_query(req, "start-after")?;

        Ok(ListObjectsV2Input {
            bucket,
            continuation_token,
            delimiter,
            encoding_type,
            expected_bucket_owner,
            fetch_owner,
            max_keys,
            prefix,
            request_payer,
            start_after,
        })
    }

    pub fn serialize_http(x: ListObjectsV2Output) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListObjectsV2 {
    fn name(&self) -> &'static str {
        "ListObjectsV2"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_objects_v2(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct ListParts;

impl ListParts {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<ListPartsInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let max_parts: MaxParts = http::parse_opt_query(req, "max-parts")?.unwrap_or(0);

        let part_number_marker: Option<PartNumberMarker> = http::parse_opt_query(req, "part-number-marker")?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let upload_id: MultipartUploadId = http::parse_query(req, "uploadId")?;

        Ok(ListPartsInput {
            bucket,
            expected_bucket_owner,
            key,
            max_parts,
            part_number_marker,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            upload_id,
        })
    }

    pub fn serialize_http(x: ListPartsOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::set_xml_body(&mut res, &x)?;
        http::add_opt_header_timestamp(&mut res, X_AMZ_ABORT_DATE, x.abort_date, TimestampFormat::HttpDate)?;
        http::add_opt_header(&mut res, X_AMZ_ABORT_RULE_ID, x.abort_rule_id)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for ListParts {
    fn name(&self) -> &'static str {
        "ListParts"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.list_parts(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketAccelerateConfiguration;

impl PutBucketAccelerateConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketAccelerateConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let accelerate_configuration: AccelerateConfiguration = http::take_xml_body(req)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(PutBucketAccelerateConfigurationInput {
            accelerate_configuration,
            bucket,
            checksum_algorithm,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: PutBucketAccelerateConfigurationOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketAccelerateConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketAccelerateConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_accelerate_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketAcl;

impl PutBucketAcl {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketAclInput> {
        let bucket = http::unwrap_bucket(req);

        let acl: Option<BucketCannedACL> = http::parse_opt_header(req, &X_AMZ_ACL)?;

        let access_control_policy: Option<AccessControlPolicy> = http::take_opt_xml_body(req)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_opt_header(req, &X_AMZ_GRANT_FULL_CONTROL)?;

        let grant_read: Option<GrantRead> = http::parse_opt_header(req, &X_AMZ_GRANT_READ)?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_opt_header(req, &X_AMZ_GRANT_READ_ACP)?;

        let grant_write: Option<GrantWrite> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE)?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE_ACP)?;

        Ok(PutBucketAclInput {
            acl,
            access_control_policy,
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write,
            grant_write_acp,
        })
    }

    pub fn serialize_http(_: PutBucketAclOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketAcl {
    fn name(&self) -> &'static str {
        "PutBucketAcl"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_acl(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketAnalyticsConfiguration;

impl PutBucketAnalyticsConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketAnalyticsConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let analytics_configuration: AnalyticsConfiguration = http::take_xml_body(req)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: AnalyticsId = http::parse_query(req, "id")?;

        Ok(PutBucketAnalyticsConfigurationInput {
            analytics_configuration,
            bucket,
            expected_bucket_owner,
            id,
        })
    }

    pub fn serialize_http(_: PutBucketAnalyticsConfigurationOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketAnalyticsConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketAnalyticsConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_analytics_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketCors;

impl PutBucketCors {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketCorsInput> {
        let bucket = http::unwrap_bucket(req);

        let cors_configuration: CORSConfiguration = http::take_xml_body(req)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(PutBucketCorsInput {
            bucket,
            cors_configuration,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: PutBucketCorsOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketCors {
    fn name(&self) -> &'static str {
        "PutBucketCors"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_cors(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketEncryption;

impl PutBucketEncryption {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketEncryptionInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let server_side_encryption_configuration: ServerSideEncryptionConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketEncryptionInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            server_side_encryption_configuration,
        })
    }

    pub fn serialize_http(_: PutBucketEncryptionOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketEncryption {
    fn name(&self) -> &'static str {
        "PutBucketEncryption"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_encryption(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketIntelligentTieringConfiguration;

impl PutBucketIntelligentTieringConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketIntelligentTieringConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let id: IntelligentTieringId = http::parse_query(req, "id")?;

        let intelligent_tiering_configuration: IntelligentTieringConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketIntelligentTieringConfigurationInput {
            bucket,
            id,
            intelligent_tiering_configuration,
        })
    }

    pub fn serialize_http(_: PutBucketIntelligentTieringConfigurationOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketIntelligentTieringConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketIntelligentTieringConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_intelligent_tiering_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketInventoryConfiguration;

impl PutBucketInventoryConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketInventoryConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: InventoryId = http::parse_query(req, "id")?;

        let inventory_configuration: InventoryConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketInventoryConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
            inventory_configuration,
        })
    }

    pub fn serialize_http(_: PutBucketInventoryConfigurationOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketInventoryConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketInventoryConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_inventory_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketLifecycleConfiguration;

impl PutBucketLifecycleConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketLifecycleConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let lifecycle_configuration: Option<BucketLifecycleConfiguration> = http::take_opt_xml_body(req)?;

        Ok(PutBucketLifecycleConfigurationInput {
            bucket,
            checksum_algorithm,
            expected_bucket_owner,
            lifecycle_configuration,
        })
    }

    pub fn serialize_http(_: PutBucketLifecycleConfigurationOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketLifecycleConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketLifecycleConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_lifecycle_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketLogging;

impl PutBucketLogging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketLoggingInput> {
        let bucket = http::unwrap_bucket(req);

        let bucket_logging_status: BucketLoggingStatus = http::take_xml_body(req)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        Ok(PutBucketLoggingInput {
            bucket,
            bucket_logging_status,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
        })
    }

    pub fn serialize_http(_: PutBucketLoggingOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketLogging {
    fn name(&self) -> &'static str {
        "PutBucketLogging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_logging(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketMetricsConfiguration;

impl PutBucketMetricsConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketMetricsConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let id: MetricsId = http::parse_query(req, "id")?;

        let metrics_configuration: MetricsConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketMetricsConfigurationInput {
            bucket,
            expected_bucket_owner,
            id,
            metrics_configuration,
        })
    }

    pub fn serialize_http(_: PutBucketMetricsConfigurationOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketMetricsConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketMetricsConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_metrics_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketNotificationConfiguration;

impl PutBucketNotificationConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketNotificationConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let notification_configuration: NotificationConfiguration = http::take_xml_body(req)?;

        let skip_destination_validation: SkipValidation =
            http::parse_opt_header(req, &X_AMZ_SKIP_DESTINATION_VALIDATION)?.unwrap_or(false);

        Ok(PutBucketNotificationConfigurationInput {
            bucket,
            expected_bucket_owner,
            notification_configuration,
            skip_destination_validation,
        })
    }

    pub fn serialize_http(_: PutBucketNotificationConfigurationOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketNotificationConfiguration {
    fn name(&self) -> &'static str {
        "PutBucketNotificationConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_notification_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketOwnershipControls;

impl PutBucketOwnershipControls {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketOwnershipControlsInput> {
        let bucket = http::unwrap_bucket(req);

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let ownership_controls: OwnershipControls = http::take_xml_body(req)?;

        Ok(PutBucketOwnershipControlsInput {
            bucket,
            content_md5,
            expected_bucket_owner,
            ownership_controls,
        })
    }

    pub fn serialize_http(_: PutBucketOwnershipControlsOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketOwnershipControls {
    fn name(&self) -> &'static str {
        "PutBucketOwnershipControls"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_ownership_controls(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketPolicy;

impl PutBucketPolicy {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketPolicyInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let confirm_remove_self_bucket_access: ConfirmRemoveSelfBucketAccess =
            http::parse_opt_header(req, &X_AMZ_CONFIRM_REMOVE_SELF_BUCKET_ACCESS)?.unwrap_or(false);

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let policy: Policy = http::take_string_body(req)?;

        Ok(PutBucketPolicyInput {
            bucket,
            checksum_algorithm,
            confirm_remove_self_bucket_access,
            content_md5,
            expected_bucket_owner,
            policy,
        })
    }

    pub fn serialize_http(_: PutBucketPolicyOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketPolicy {
    fn name(&self) -> &'static str {
        "PutBucketPolicy"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_policy(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketReplication;

impl PutBucketReplication {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketReplicationInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let replication_configuration: ReplicationConfiguration = http::take_xml_body(req)?;

        let token: Option<ObjectLockToken> = http::parse_opt_header(req, &X_AMZ_BUCKET_OBJECT_LOCK_TOKEN)?;

        Ok(PutBucketReplicationInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            replication_configuration,
            token,
        })
    }

    pub fn serialize_http(_: PutBucketReplicationOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketReplication {
    fn name(&self) -> &'static str {
        "PutBucketReplication"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_replication(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketRequestPayment;

impl PutBucketRequestPayment {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketRequestPaymentInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payment_configuration: RequestPaymentConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketRequestPaymentInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            request_payment_configuration,
        })
    }

    pub fn serialize_http(_: PutBucketRequestPaymentOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketRequestPayment {
    fn name(&self) -> &'static str {
        "PutBucketRequestPayment"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_request_payment(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketTagging;

impl PutBucketTagging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketTaggingInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let tagging: Tagging = http::take_xml_body(req)?;

        Ok(PutBucketTaggingInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            tagging,
        })
    }

    pub fn serialize_http(_: PutBucketTaggingOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketTagging {
    fn name(&self) -> &'static str {
        "PutBucketTagging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_tagging(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketVersioning;

impl PutBucketVersioning {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketVersioningInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let mfa: Option<MFA> = http::parse_opt_header(req, &X_AMZ_MFA)?;

        let versioning_configuration: VersioningConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketVersioningInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            mfa,
            versioning_configuration,
        })
    }

    pub fn serialize_http(_: PutBucketVersioningOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketVersioning {
    fn name(&self) -> &'static str {
        "PutBucketVersioning"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_versioning(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutBucketWebsite;

impl PutBucketWebsite {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutBucketWebsiteInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let website_configuration: WebsiteConfiguration = http::take_xml_body(req)?;

        Ok(PutBucketWebsiteInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            website_configuration,
        })
    }

    pub fn serialize_http(_: PutBucketWebsiteOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutBucketWebsite {
    fn name(&self) -> &'static str {
        "PutBucketWebsite"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_bucket_website(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutObject;

impl PutObject {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutObjectInput> {
        if let Some(m) = req.extensions_mut().remove::<http::Multipart>() {
            return Self::deserialize_http_multipart(req, m);
        }

        let (bucket, key) = http::unwrap_object(req);

        let acl: Option<ObjectCannedACL> = http::parse_opt_header(req, &X_AMZ_ACL)?;

        let body: Option<StreamingBlob> = Some(http::take_stream_body(req));

        let bucket_key_enabled: BucketKeyEnabled =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED)?.unwrap_or(false);

        let cache_control: Option<CacheControl> = http::parse_opt_header(req, &CACHE_CONTROL)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let checksum_crc32: Option<ChecksumCRC32> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_CRC32)?;

        let checksum_crc32c: Option<ChecksumCRC32C> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_CRC32C)?;

        let checksum_sha1: Option<ChecksumSHA1> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_SHA1)?;

        let checksum_sha256: Option<ChecksumSHA256> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_SHA256)?;

        let content_disposition: Option<ContentDisposition> = http::parse_opt_header(req, &CONTENT_DISPOSITION)?;

        let content_encoding: Option<ContentEncoding> = http::parse_opt_header(req, &CONTENT_ENCODING)?;

        let content_language: Option<ContentLanguage> = http::parse_opt_header(req, &CONTENT_LANGUAGE)?;

        let content_length: ContentLength = http::parse_opt_header(req, &CONTENT_LENGTH)?.unwrap_or(0);

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let content_type: Option<ContentType> = http::parse_opt_header(req, &CONTENT_TYPE)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let expires: Option<Expires> = http::parse_opt_header_timestamp(req, &EXPIRES, TimestampFormat::HttpDate)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_opt_header(req, &X_AMZ_GRANT_FULL_CONTROL)?;

        let grant_read: Option<GrantRead> = http::parse_opt_header(req, &X_AMZ_GRANT_READ)?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_opt_header(req, &X_AMZ_GRANT_READ_ACP)?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE_ACP)?;

        let metadata: Option<Metadata> = http::parse_opt_metadata(req)?;

        let object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus> =
            http::parse_opt_header(req, &X_AMZ_OBJECT_LOCK_LEGAL_HOLD)?;

        let object_lock_mode: Option<ObjectLockMode> = http::parse_opt_header(req, &X_AMZ_OBJECT_LOCK_MODE)?;

        let object_lock_retain_until_date: Option<ObjectLockRetainUntilDate> =
            http::parse_opt_header_timestamp(req, &X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE, TimestampFormat::DateTime)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let ssekms_encryption_context: Option<SSEKMSEncryptionContext> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT)?;

        let ssekms_key_id: Option<SSEKMSKeyId> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID)?;

        let server_side_encryption: Option<ServerSideEncryption> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION)?;

        let storage_class: Option<StorageClass> = http::parse_opt_header(req, &X_AMZ_STORAGE_CLASS)?;

        let tagging: Option<TaggingHeader> = http::parse_opt_header(req, &X_AMZ_TAGGING)?;

        let website_redirect_location: Option<WebsiteRedirectLocation> =
            http::parse_opt_header(req, &X_AMZ_WEBSITE_REDIRECT_LOCATION)?;

        Ok(PutObjectInput {
            acl,
            body,
            bucket,
            bucket_key_enabled,
            cache_control,
            checksum_algorithm,
            checksum_crc32,
            checksum_crc32c,
            checksum_sha1,
            checksum_sha256,
            content_disposition,
            content_encoding,
            content_language,
            content_length,
            content_md5,
            content_type,
            expected_bucket_owner,
            expires,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write_acp,
            key,
            metadata,
            object_lock_legal_hold_status,
            object_lock_mode,
            object_lock_retain_until_date,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            ssekms_encryption_context,
            ssekms_key_id,
            server_side_encryption,
            storage_class,
            tagging,
            website_redirect_location,
        })
    }

    pub fn deserialize_http_multipart(req: &mut http::Request, mut m: http::Multipart) -> S3Result<PutObjectInput> {
        let bucket = http::unwrap_bucket(req);
        let key = http::parse_field_value(&m, "key")?.ok_or_else(|| invalid_request!("missing key"))?;

        let body: Option<StreamingBlob> = m.take_file_stream().map(StreamingBlob::wrap);

        let acl: Option<ObjectCannedACL> = http::parse_field_value(&m, "x-amz-acl")?;

        let bucket_key_enabled: BucketKeyEnabled =
            http::parse_field_value(&m, "x-amz-server-side-encryption-bucket-key-enabled")?.unwrap_or(false);

        let cache_control: Option<CacheControl> = http::parse_field_value(&m, "cache-control")?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_field_value(&m, "x-amz-sdk-checksum-algorithm")?;

        let checksum_crc32: Option<ChecksumCRC32> = http::parse_field_value(&m, "x-amz-checksum-crc32")?;

        let checksum_crc32c: Option<ChecksumCRC32C> = http::parse_field_value(&m, "x-amz-checksum-crc32c")?;

        let checksum_sha1: Option<ChecksumSHA1> = http::parse_field_value(&m, "x-amz-checksum-sha1")?;

        let checksum_sha256: Option<ChecksumSHA256> = http::parse_field_value(&m, "x-amz-checksum-sha256")?;

        let content_disposition: Option<ContentDisposition> = http::parse_field_value(&m, "content-disposition")?;

        let content_encoding: Option<ContentEncoding> = http::parse_field_value(&m, "content-encoding")?;

        let content_language: Option<ContentLanguage> = http::parse_field_value(&m, "content-language")?;

        let content_length: ContentLength = http::parse_field_value(&m, "content-length")?.unwrap_or(0);

        let content_md5: Option<ContentMD5> = http::parse_field_value(&m, "content-md5")?;

        let content_type: Option<ContentType> = http::parse_field_value(&m, "content-type")?;

        let expected_bucket_owner: Option<AccountId> = http::parse_field_value(&m, "x-amz-expected-bucket-owner")?;

        let expires: Option<Expires> = http::parse_field_value_timestamp(&m, "expires", TimestampFormat::HttpDate)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_field_value(&m, "x-amz-grant-full-control")?;

        let grant_read: Option<GrantRead> = http::parse_field_value(&m, "x-amz-grant-read")?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_field_value(&m, "x-amz-grant-read-acp")?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_field_value(&m, "x-amz-grant-write-acp")?;

        let metadata: Option<Metadata> = {
            let mut metadata: Metadata = Default::default();
            for (name, value) in m.fields() {
                if let Some(key) = name.strip_prefix("x-amz-meta-") {
                    if key.is_empty() {
                        continue;
                    }
                    metadata.insert(key.to_owned(), value.to_owned());
                }
            }
            if metadata.is_empty() {
                None
            } else {
                Some(metadata)
            }
        };

        let object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus> =
            http::parse_field_value(&m, "x-amz-object-lock-legal-hold")?;

        let object_lock_mode: Option<ObjectLockMode> = http::parse_field_value(&m, "x-amz-object-lock-mode")?;

        let object_lock_retain_until_date: Option<ObjectLockRetainUntilDate> =
            http::parse_field_value_timestamp(&m, "x-amz-object-lock-retain-until-date", TimestampFormat::DateTime)?;

        let request_payer: Option<RequestPayer> = http::parse_field_value(&m, "x-amz-request-payer")?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_field_value(&m, "x-amz-server-side-encryption-customer-algorithm")?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_field_value(&m, "x-amz-server-side-encryption-customer-key")?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_field_value(&m, "x-amz-server-side-encryption-customer-key-md5")?;

        let ssekms_encryption_context: Option<SSEKMSEncryptionContext> =
            http::parse_field_value(&m, "x-amz-server-side-encryption-context")?;

        let ssekms_key_id: Option<SSEKMSKeyId> = http::parse_field_value(&m, "x-amz-server-side-encryption-aws-kms-key-id")?;

        let server_side_encryption: Option<ServerSideEncryption> = http::parse_field_value(&m, "x-amz-server-side-encryption")?;

        let storage_class: Option<StorageClass> = http::parse_field_value(&m, "x-amz-storage-class")?;

        let tagging: Option<TaggingHeader> = http::parse_field_value(&m, "x-amz-tagging")?;

        let website_redirect_location: Option<WebsiteRedirectLocation> =
            http::parse_field_value(&m, "x-amz-website-redirect-location")?;

        Ok(PutObjectInput {
            acl,
            body,
            bucket,
            bucket_key_enabled,
            cache_control,
            checksum_algorithm,
            checksum_crc32,
            checksum_crc32c,
            checksum_sha1,
            checksum_sha256,
            content_disposition,
            content_encoding,
            content_language,
            content_length,
            content_md5,
            content_type,
            expected_bucket_owner,
            expires,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write_acp,
            key,
            metadata,
            object_lock_legal_hold_status,
            object_lock_mode,
            object_lock_retain_until_date,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            ssekms_encryption_context,
            ssekms_key_id,
            server_side_encryption,
            storage_class,
            tagging,
            website_redirect_location,
        })
    }

    pub fn serialize_http(x: PutObjectOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED, x.bucket_key_enabled)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32, x.checksum_crc32)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32C, x.checksum_crc32c)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA1, x.checksum_sha1)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA256, x.checksum_sha256)?;
        http::add_opt_header(&mut res, ETAG, x.e_tag)?;
        http::add_opt_header(&mut res, X_AMZ_EXPIRATION, x.expiration)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM, x.sse_customer_algorithm)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5, x.sse_customer_key_md5)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CONTEXT, x.ssekms_encryption_context)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutObject {
    fn name(&self) -> &'static str {
        "PutObject"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_object(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutObjectAcl;

impl PutObjectAcl {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutObjectAclInput> {
        let (bucket, key) = http::unwrap_object(req);

        let acl: Option<ObjectCannedACL> = http::parse_opt_header(req, &X_AMZ_ACL)?;

        let access_control_policy: Option<AccessControlPolicy> = http::take_opt_xml_body(req)?;

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let grant_full_control: Option<GrantFullControl> = http::parse_opt_header(req, &X_AMZ_GRANT_FULL_CONTROL)?;

        let grant_read: Option<GrantRead> = http::parse_opt_header(req, &X_AMZ_GRANT_READ)?;

        let grant_read_acp: Option<GrantReadACP> = http::parse_opt_header(req, &X_AMZ_GRANT_READ_ACP)?;

        let grant_write: Option<GrantWrite> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE)?;

        let grant_write_acp: Option<GrantWriteACP> = http::parse_opt_header(req, &X_AMZ_GRANT_WRITE_ACP)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(PutObjectAclInput {
            acl,
            access_control_policy,
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            grant_full_control,
            grant_read,
            grant_read_acp,
            grant_write,
            grant_write_acp,
            key,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: PutObjectAclOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutObjectAcl {
    fn name(&self) -> &'static str {
        "PutObjectAcl"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_object_acl(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutObjectLegalHold;

impl PutObjectLegalHold {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutObjectLegalHoldInput> {
        let (bucket, key) = http::unwrap_object(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let legal_hold: Option<ObjectLockLegalHold> = http::take_opt_xml_body(req)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(PutObjectLegalHoldInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            key,
            legal_hold,
            request_payer,
            version_id,
        })
    }

    pub fn serialize_http(x: PutObjectLegalHoldOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutObjectLegalHold {
    fn name(&self) -> &'static str {
        "PutObjectLegalHold"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_object_legal_hold(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutObjectLockConfiguration;

impl PutObjectLockConfiguration {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutObjectLockConfigurationInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let object_lock_configuration: Option<ObjectLockConfiguration> = http::take_opt_xml_body(req)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let token: Option<ObjectLockToken> = http::parse_opt_header(req, &X_AMZ_BUCKET_OBJECT_LOCK_TOKEN)?;

        Ok(PutObjectLockConfigurationInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            object_lock_configuration,
            request_payer,
            token,
        })
    }

    pub fn serialize_http(x: PutObjectLockConfigurationOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutObjectLockConfiguration {
    fn name(&self) -> &'static str {
        "PutObjectLockConfiguration"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_object_lock_configuration(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutObjectRetention;

impl PutObjectRetention {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutObjectRetentionInput> {
        let (bucket, key) = http::unwrap_object(req);

        let bypass_governance_retention: BypassGovernanceRetention =
            http::parse_opt_header(req, &X_AMZ_BYPASS_GOVERNANCE_RETENTION)?.unwrap_or(false);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let retention: Option<ObjectLockRetention> = http::take_opt_xml_body(req)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(PutObjectRetentionInput {
            bucket,
            bypass_governance_retention,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            key,
            request_payer,
            retention,
            version_id,
        })
    }

    pub fn serialize_http(x: PutObjectRetentionOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutObjectRetention {
    fn name(&self) -> &'static str {
        "PutObjectRetention"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_object_retention(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutObjectTagging;

impl PutObjectTagging {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutObjectTaggingInput> {
        let (bucket, key) = http::unwrap_object(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let tagging: Tagging = http::take_xml_body(req)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(PutObjectTaggingInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            key,
            request_payer,
            tagging,
            version_id,
        })
    }

    pub fn serialize_http(x: PutObjectTaggingOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, X_AMZ_VERSION_ID, x.version_id)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for PutObjectTagging {
    fn name(&self) -> &'static str {
        "PutObjectTagging"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_object_tagging(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct PutPublicAccessBlock;

impl PutPublicAccessBlock {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<PutPublicAccessBlockInput> {
        let bucket = http::unwrap_bucket(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let public_access_block_configuration: PublicAccessBlockConfiguration = http::take_xml_body(req)?;

        Ok(PutPublicAccessBlockInput {
            bucket,
            checksum_algorithm,
            content_md5,
            expected_bucket_owner,
            public_access_block_configuration,
        })
    }

    pub fn serialize_http(_: PutPublicAccessBlockOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for PutPublicAccessBlock {
    fn name(&self) -> &'static str {
        "PutPublicAccessBlock"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.put_public_access_block(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct RestoreObject;

impl RestoreObject {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<RestoreObjectInput> {
        let (bucket, key) = http::unwrap_object(req);

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let restore_request: Option<RestoreRequest> = http::take_opt_xml_body(req)?;

        let version_id: Option<ObjectVersionId> = http::parse_opt_query(req, "versionId")?;

        Ok(RestoreObjectInput {
            bucket,
            checksum_algorithm,
            expected_bucket_owner,
            key,
            request_payer,
            restore_request,
            version_id,
        })
    }

    pub fn serialize_http(x: RestoreObjectOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_RESTORE_OUTPUT_PATH, x.restore_output_path)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for RestoreObject {
    fn name(&self) -> &'static str {
        "RestoreObject"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.restore_object(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

#[allow(dead_code)] // TODO
pub struct SelectObjectContent;

#[allow(dead_code)] // TODO
impl SelectObjectContent {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<SelectObjectContentInput> {
        let (bucket, key) = http::unwrap_object(req);

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let request: SelectObjectContentRequest = http::take_xml_body(req)?;

        Ok(SelectObjectContentInput {
            bucket,
            expected_bucket_owner,
            key,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            request,
        })
    }
}

pub struct UploadPart;

impl UploadPart {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<UploadPartInput> {
        let (bucket, key) = http::unwrap_object(req);

        let body: Option<StreamingBlob> = Some(http::take_stream_body(req));

        let checksum_algorithm: Option<ChecksumAlgorithm> = http::parse_opt_header(req, &X_AMZ_SDK_CHECKSUM_ALGORITHM)?;

        let checksum_crc32: Option<ChecksumCRC32> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_CRC32)?;

        let checksum_crc32c: Option<ChecksumCRC32C> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_CRC32C)?;

        let checksum_sha1: Option<ChecksumSHA1> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_SHA1)?;

        let checksum_sha256: Option<ChecksumSHA256> = http::parse_opt_header(req, &X_AMZ_CHECKSUM_SHA256)?;

        let content_length: ContentLength = http::parse_opt_header(req, &CONTENT_LENGTH)?.unwrap_or(0);

        let content_md5: Option<ContentMD5> = http::parse_opt_header(req, &CONTENT_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let part_number: PartNumber = http::parse_opt_query(req, "partNumber")?.unwrap_or(0);

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let upload_id: MultipartUploadId = http::parse_query(req, "uploadId")?;

        Ok(UploadPartInput {
            body,
            bucket,
            checksum_algorithm,
            checksum_crc32,
            checksum_crc32c,
            checksum_sha1,
            checksum_sha256,
            content_length,
            content_md5,
            expected_bucket_owner,
            key,
            part_number,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            upload_id,
        })
    }

    pub fn serialize_http(x: UploadPartOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        http::add_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED, x.bucket_key_enabled)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32, x.checksum_crc32)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_CRC32C, x.checksum_crc32c)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA1, x.checksum_sha1)?;
        http::add_opt_header(&mut res, X_AMZ_CHECKSUM_SHA256, x.checksum_sha256)?;
        http::add_opt_header(&mut res, ETAG, x.e_tag)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM, x.sse_customer_algorithm)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5, x.sse_customer_key_md5)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for UploadPart {
    fn name(&self) -> &'static str {
        "UploadPart"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.upload_part(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct UploadPartCopy;

impl UploadPartCopy {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<UploadPartCopyInput> {
        let (bucket, key) = http::unwrap_object(req);

        let copy_source: CopySource = http::parse_header(req, &X_AMZ_COPY_SOURCE)?;

        let copy_source_if_match: Option<CopySourceIfMatch> = http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_IF_MATCH)?;

        let copy_source_if_modified_since: Option<CopySourceIfModifiedSince> =
            http::parse_opt_header_timestamp(req, &X_AMZ_COPY_SOURCE_IF_MODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let copy_source_if_none_match: Option<CopySourceIfNoneMatch> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_IF_NONE_MATCH)?;

        let copy_source_if_unmodified_since: Option<CopySourceIfUnmodifiedSince> =
            http::parse_opt_header_timestamp(req, &X_AMZ_COPY_SOURCE_IF_UNMODIFIED_SINCE, TimestampFormat::HttpDate)?;

        let copy_source_range: Option<CopySourceRange> = http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_RANGE)?;

        let copy_source_sse_customer_algorithm: Option<CopySourceSSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let copy_source_sse_customer_key: Option<CopySourceSSECustomerKey> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let copy_source_sse_customer_key_md5: Option<CopySourceSSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_COPY_SOURCE_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let expected_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_EXPECTED_BUCKET_OWNER)?;

        let expected_source_bucket_owner: Option<AccountId> = http::parse_opt_header(req, &X_AMZ_SOURCE_EXPECTED_BUCKET_OWNER)?;

        let part_number: PartNumber = http::parse_opt_query(req, "partNumber")?.unwrap_or(0);

        let request_payer: Option<RequestPayer> = http::parse_opt_header(req, &X_AMZ_REQUEST_PAYER)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key: Option<SSECustomerKey> = http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let upload_id: MultipartUploadId = http::parse_query(req, "uploadId")?;

        Ok(UploadPartCopyInput {
            bucket,
            copy_source,
            copy_source_if_match,
            copy_source_if_modified_since,
            copy_source_if_none_match,
            copy_source_if_unmodified_since,
            copy_source_range,
            copy_source_sse_customer_algorithm,
            copy_source_sse_customer_key,
            copy_source_sse_customer_key_md5,
            expected_bucket_owner,
            expected_source_bucket_owner,
            key,
            part_number,
            request_payer,
            sse_customer_algorithm,
            sse_customer_key,
            sse_customer_key_md5,
            upload_id,
        })
    }

    pub fn serialize_http(x: UploadPartCopyOutput) -> S3Result<http::Response> {
        let mut res = http::Response::default();
        if let Some(ref val) = x.copy_part_result {
            http::set_xml_body(&mut res, val)?;
        }
        http::add_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED, x.bucket_key_enabled)?;
        http::add_opt_header(&mut res, X_AMZ_COPY_SOURCE_VERSION_ID, x.copy_source_version_id)?;
        http::add_opt_header(&mut res, X_AMZ_REQUEST_CHARGED, x.request_charged)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM, x.sse_customer_algorithm)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5, x.sse_customer_key_md5)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID, x.ssekms_key_id)?;
        http::add_opt_header(&mut res, X_AMZ_SERVER_SIDE_ENCRYPTION, x.server_side_encryption)?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl super::Operation for UploadPartCopy {
    fn name(&self) -> &'static str {
        "UploadPartCopy"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.upload_part_copy(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub struct WriteGetObjectResponse;

impl WriteGetObjectResponse {
    pub fn deserialize_http(req: &mut http::Request) -> S3Result<WriteGetObjectResponseInput> {
        let accept_ranges: Option<AcceptRanges> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_ACCEPT_RANGES)?;

        let body: Option<StreamingBlob> = Some(http::take_stream_body(req));

        let bucket_key_enabled: BucketKeyEnabled =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION_BUCKET_KEY_ENABLED)?.unwrap_or(false);

        let cache_control: Option<CacheControl> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_CACHE_CONTROL)?;

        let checksum_crc32: Option<ChecksumCRC32> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_CHECKSUM_CRC32)?;

        let checksum_crc32c: Option<ChecksumCRC32C> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_CHECKSUM_CRC32C)?;

        let checksum_sha1: Option<ChecksumSHA1> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_CHECKSUM_SHA1)?;

        let checksum_sha256: Option<ChecksumSHA256> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_CHECKSUM_SHA256)?;

        let content_disposition: Option<ContentDisposition> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_CONTENT_DISPOSITION)?;

        let content_encoding: Option<ContentEncoding> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_CONTENT_ENCODING)?;

        let content_language: Option<ContentLanguage> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_CONTENT_LANGUAGE)?;

        let content_length: ContentLength = http::parse_opt_header(req, &CONTENT_LENGTH)?.unwrap_or(0);

        let content_range: Option<ContentRange> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_CONTENT_RANGE)?;

        let content_type: Option<ContentType> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_CONTENT_TYPE)?;

        let delete_marker: DeleteMarker = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_DELETE_MARKER)?.unwrap_or(false);

        let e_tag: Option<ETag> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_E_TAG)?;

        let error_code: Option<ErrorCode> = http::parse_opt_header(req, &X_AMZ_FWD_ERROR_CODE)?;

        let error_message: Option<ErrorMessage> = http::parse_opt_header(req, &X_AMZ_FWD_ERROR_MESSAGE)?;

        let expiration: Option<Expiration> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_EXPIRATION)?;

        let expires: Option<Expires> =
            http::parse_opt_header_timestamp(req, &X_AMZ_FWD_HEADER_EXPIRES, TimestampFormat::HttpDate)?;

        let last_modified: Option<LastModified> =
            http::parse_opt_header_timestamp(req, &X_AMZ_FWD_HEADER_LAST_MODIFIED, TimestampFormat::HttpDate)?;

        let metadata: Option<Metadata> = http::parse_opt_metadata(req)?;

        let missing_meta: MissingMeta = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_MISSING_META)?.unwrap_or(0);

        let object_lock_legal_hold_status: Option<ObjectLockLegalHoldStatus> =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_OBJECT_LOCK_LEGAL_HOLD)?;

        let object_lock_mode: Option<ObjectLockMode> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_OBJECT_LOCK_MODE)?;

        let object_lock_retain_until_date: Option<ObjectLockRetainUntilDate> = http::parse_opt_header_timestamp(
            req,
            &X_AMZ_FWD_HEADER_X_AMZ_OBJECT_LOCK_RETAIN_UNTIL_DATE,
            TimestampFormat::DateTime,
        )?;

        let parts_count: PartsCount = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_MP_PARTS_COUNT)?.unwrap_or(0);

        let replication_status: Option<ReplicationStatus> =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_REPLICATION_STATUS)?;

        let request_charged: Option<RequestCharged> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_REQUEST_CHARGED)?;

        let request_route: RequestRoute = http::parse_header(req, &X_AMZ_REQUEST_ROUTE)?;

        let request_token: RequestToken = http::parse_header(req, &X_AMZ_REQUEST_TOKEN)?;

        let restore: Option<Restore> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_RESTORE)?;

        let sse_customer_algorithm: Option<SSECustomerAlgorithm> =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_ALGORITHM)?;

        let sse_customer_key_md5: Option<SSECustomerKeyMD5> =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION_CUSTOMER_KEY_MD5)?;

        let ssekms_key_id: Option<SSEKMSKeyId> =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION_AWS_KMS_KEY_ID)?;

        let server_side_encryption: Option<ServerSideEncryption> =
            http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_SERVER_SIDE_ENCRYPTION)?;

        let status_code: GetObjectResponseStatusCode = http::parse_opt_header(req, &X_AMZ_FWD_STATUS)?.unwrap_or(0);

        let storage_class: Option<StorageClass> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_STORAGE_CLASS)?;

        let tag_count: TagCount = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_TAGGING_COUNT)?.unwrap_or(0);

        let version_id: Option<ObjectVersionId> = http::parse_opt_header(req, &X_AMZ_FWD_HEADER_X_AMZ_VERSION_ID)?;

        Ok(WriteGetObjectResponseInput {
            accept_ranges,
            body,
            bucket_key_enabled,
            cache_control,
            checksum_crc32,
            checksum_crc32c,
            checksum_sha1,
            checksum_sha256,
            content_disposition,
            content_encoding,
            content_language,
            content_length,
            content_range,
            content_type,
            delete_marker,
            e_tag,
            error_code,
            error_message,
            expiration,
            expires,
            last_modified,
            metadata,
            missing_meta,
            object_lock_legal_hold_status,
            object_lock_mode,
            object_lock_retain_until_date,
            parts_count,
            replication_status,
            request_charged,
            request_route,
            request_token,
            restore,
            sse_customer_algorithm,
            sse_customer_key_md5,
            ssekms_key_id,
            server_side_encryption,
            status_code,
            storage_class,
            tag_count,
            version_id,
        })
    }

    pub fn serialize_http(_: WriteGetObjectResponseOutput) -> S3Result<http::Response> {
        Ok(http::Response::default())
    }
}

#[async_trait::async_trait]
impl super::Operation for WriteGetObjectResponse {
    fn name(&self) -> &'static str {
        "WriteGetObjectResponse"
    }

    async fn call(&self, s3: &dyn S3, req: &mut http::Request) -> S3Result<http::Response> {
        let input = Self::deserialize_http(req)?;
        let result = s3.write_get_object_response(input).await;
        let res = match result {
            Ok(output) => Self::serialize_http(output)?,
            Err(err) => super::serialize_error(err)?,
        };
        Ok(res)
    }
}

pub fn resolve_route(
    req: &http::Request,
    s3_path: &S3Path,
    qs: Option<&http::OrderedQs>,
) -> S3Result<(&'static dyn super::Operation, bool)> {
    match req.method().clone() {
        hyper::Method::HEAD => match s3_path {
            S3Path::Root => Err(super::unknown_operation()),
            S3Path::Bucket { .. } => Ok((&HeadBucket as &'static dyn super::Operation, false)),
            S3Path::Object { .. } => Ok((&HeadObject as &'static dyn super::Operation, false)),
        },
        hyper::Method::GET => match s3_path {
            S3Path::Root => Ok((&ListBuckets as &'static dyn super::Operation, false)),
            S3Path::Bucket { .. } => {
                if let Some(qs) = qs {
                    if qs.has("analytics") {
                        return Ok((&GetBucketAnalyticsConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("intelligent-tiering") {
                        return Ok((&GetBucketIntelligentTieringConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("inventory") {
                        return Ok((&GetBucketInventoryConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("metrics") {
                        return Ok((&GetBucketMetricsConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("accelerate") {
                        return Ok((&GetBucketAccelerateConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("acl") {
                        return Ok((&GetBucketAcl as &'static dyn super::Operation, false));
                    }
                    if qs.has("cors") {
                        return Ok((&GetBucketCors as &'static dyn super::Operation, false));
                    }
                    if qs.has("encryption") {
                        return Ok((&GetBucketEncryption as &'static dyn super::Operation, false));
                    }
                    if qs.has("lifecycle") {
                        return Ok((&GetBucketLifecycleConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("location") {
                        return Ok((&GetBucketLocation as &'static dyn super::Operation, false));
                    }
                    if qs.has("logging") {
                        return Ok((&GetBucketLogging as &'static dyn super::Operation, false));
                    }
                    if qs.has("notification") {
                        return Ok((&GetBucketNotificationConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("ownershipControls") {
                        return Ok((&GetBucketOwnershipControls as &'static dyn super::Operation, false));
                    }
                    if qs.has("policy") {
                        return Ok((&GetBucketPolicy as &'static dyn super::Operation, false));
                    }
                    if qs.has("policyStatus") {
                        return Ok((&GetBucketPolicyStatus as &'static dyn super::Operation, false));
                    }
                    if qs.has("replication") {
                        return Ok((&GetBucketReplication as &'static dyn super::Operation, false));
                    }
                    if qs.has("requestPayment") {
                        return Ok((&GetBucketRequestPayment as &'static dyn super::Operation, false));
                    }
                    if qs.has("tagging") {
                        return Ok((&GetBucketTagging as &'static dyn super::Operation, false));
                    }
                    if qs.has("versioning") {
                        return Ok((&GetBucketVersioning as &'static dyn super::Operation, false));
                    }
                    if qs.has("website") {
                        return Ok((&GetBucketWebsite as &'static dyn super::Operation, false));
                    }
                    if qs.has("object-lock") {
                        return Ok((&GetObjectLockConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("publicAccessBlock") {
                        return Ok((&GetPublicAccessBlock as &'static dyn super::Operation, false));
                    }
                    if qs.has("analytics") {
                        return Ok((&ListBucketAnalyticsConfigurations as &'static dyn super::Operation, false));
                    }
                    if qs.has("intelligent-tiering") {
                        return Ok((&ListBucketIntelligentTieringConfigurations as &'static dyn super::Operation, false));
                    }
                    if qs.has("inventory") {
                        return Ok((&ListBucketInventoryConfigurations as &'static dyn super::Operation, false));
                    }
                    if qs.has("metrics") {
                        return Ok((&ListBucketMetricsConfigurations as &'static dyn super::Operation, false));
                    }
                    if qs.has("uploads") {
                        return Ok((&ListMultipartUploads as &'static dyn super::Operation, false));
                    }
                    if qs.has("versions") {
                        return Ok((&ListObjectVersions as &'static dyn super::Operation, false));
                    }
                    if super::check_query_pattern(qs, "list-type", "2") {
                        return Ok((&ListObjectsV2 as &'static dyn super::Operation, false));
                    }
                }
                Ok((&ListObjects as &'static dyn super::Operation, false))
            }
            S3Path::Object { .. } => {
                if let Some(qs) = qs {
                    if qs.has("attributes") {
                        return Ok((&GetObjectAttributes as &'static dyn super::Operation, false));
                    }
                    if qs.has("acl") {
                        return Ok((&GetObjectAcl as &'static dyn super::Operation, false));
                    }
                    if qs.has("legal-hold") {
                        return Ok((&GetObjectLegalHold as &'static dyn super::Operation, false));
                    }
                    if qs.has("retention") {
                        return Ok((&GetObjectRetention as &'static dyn super::Operation, false));
                    }
                    if qs.has("tagging") {
                        return Ok((&GetObjectTagging as &'static dyn super::Operation, false));
                    }
                    if qs.has("torrent") {
                        return Ok((&GetObjectTorrent as &'static dyn super::Operation, false));
                    }
                }
                if let Some(qs) = qs {
                    if qs.has("uploadId") {
                        return Ok((&ListParts as &'static dyn super::Operation, false));
                    }
                }
                Ok((&GetObject as &'static dyn super::Operation, false))
            }
        },
        hyper::Method::POST => match s3_path {
            S3Path::Root => Err(super::unknown_operation()),
            S3Path::Bucket { .. } => {
                if let Some(qs) = qs {
                    if qs.has("delete") {
                        return Ok((&DeleteObjects as &'static dyn super::Operation, true));
                    }
                }
                if req.headers().contains_key("x-amz-request-route") && req.headers().contains_key("x-amz-request-token") {
                    return Ok((&WriteGetObjectResponse as &'static dyn super::Operation, false));
                }
                Err(super::unknown_operation())
            }
            S3Path::Object { .. } => {
                if let Some(qs) = qs {
                    if qs.has("uploads") {
                        return Ok((&CreateMultipartUpload as &'static dyn super::Operation, false));
                    }
                    if qs.has("restore") {
                        return Ok((&RestoreObject as &'static dyn super::Operation, true));
                    }
                }
                if let Some(qs) = qs {
                    if qs.has("uploadId") {
                        return Ok((&CompleteMultipartUpload as &'static dyn super::Operation, true));
                    }
                }
                Err(super::unknown_operation())
            }
        },
        hyper::Method::PUT => match s3_path {
            S3Path::Root => Err(super::unknown_operation()),
            S3Path::Bucket { .. } => {
                if let Some(qs) = qs {
                    if qs.has("analytics") {
                        return Ok((&PutBucketAnalyticsConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("intelligent-tiering") {
                        return Ok((&PutBucketIntelligentTieringConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("inventory") {
                        return Ok((&PutBucketInventoryConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("metrics") {
                        return Ok((&PutBucketMetricsConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("accelerate") {
                        return Ok((&PutBucketAccelerateConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("acl") {
                        return Ok((&PutBucketAcl as &'static dyn super::Operation, true));
                    }
                    if qs.has("cors") {
                        return Ok((&PutBucketCors as &'static dyn super::Operation, true));
                    }
                    if qs.has("encryption") {
                        return Ok((&PutBucketEncryption as &'static dyn super::Operation, true));
                    }
                    if qs.has("lifecycle") {
                        return Ok((&PutBucketLifecycleConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("logging") {
                        return Ok((&PutBucketLogging as &'static dyn super::Operation, true));
                    }
                    if qs.has("notification") {
                        return Ok((&PutBucketNotificationConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("ownershipControls") {
                        return Ok((&PutBucketOwnershipControls as &'static dyn super::Operation, true));
                    }
                    if qs.has("policy") {
                        return Ok((&PutBucketPolicy as &'static dyn super::Operation, true));
                    }
                    if qs.has("replication") {
                        return Ok((&PutBucketReplication as &'static dyn super::Operation, true));
                    }
                    if qs.has("requestPayment") {
                        return Ok((&PutBucketRequestPayment as &'static dyn super::Operation, true));
                    }
                    if qs.has("tagging") {
                        return Ok((&PutBucketTagging as &'static dyn super::Operation, true));
                    }
                    if qs.has("versioning") {
                        return Ok((&PutBucketVersioning as &'static dyn super::Operation, true));
                    }
                    if qs.has("website") {
                        return Ok((&PutBucketWebsite as &'static dyn super::Operation, true));
                    }
                    if qs.has("object-lock") {
                        return Ok((&PutObjectLockConfiguration as &'static dyn super::Operation, true));
                    }
                    if qs.has("publicAccessBlock") {
                        return Ok((&PutPublicAccessBlock as &'static dyn super::Operation, true));
                    }
                }
                Ok((&CreateBucket as &'static dyn super::Operation, true))
            }
            S3Path::Object { .. } => {
                if let Some(qs) = qs {
                    if qs.has("acl") {
                        return Ok((&PutObjectAcl as &'static dyn super::Operation, true));
                    }
                    if qs.has("legal-hold") {
                        return Ok((&PutObjectLegalHold as &'static dyn super::Operation, true));
                    }
                    if qs.has("retention") {
                        return Ok((&PutObjectRetention as &'static dyn super::Operation, true));
                    }
                    if qs.has("tagging") {
                        return Ok((&PutObjectTagging as &'static dyn super::Operation, true));
                    }
                }
                if let Some(qs) = qs {
                    if qs.has("uploadId") && req.headers().contains_key("x-amz-copy-source") {
                        return Ok((&UploadPartCopy as &'static dyn super::Operation, false));
                    }
                }
                if let Some(qs) = qs {
                    if qs.has("uploadId") {
                        return Ok((&UploadPart as &'static dyn super::Operation, false));
                    }
                }
                if req.headers().contains_key("x-amz-copy-source") {
                    return Ok((&CopyObject as &'static dyn super::Operation, false));
                }
                Ok((&PutObject as &'static dyn super::Operation, false))
            }
        },
        hyper::Method::DELETE => match s3_path {
            S3Path::Root => Err(super::unknown_operation()),
            S3Path::Bucket { .. } => {
                if let Some(qs) = qs {
                    if qs.has("analytics") {
                        return Ok((&DeleteBucketAnalyticsConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("intelligent-tiering") {
                        return Ok((&DeleteBucketIntelligentTieringConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("inventory") {
                        return Ok((&DeleteBucketInventoryConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("metrics") {
                        return Ok((&DeleteBucketMetricsConfiguration as &'static dyn super::Operation, false));
                    }
                    if qs.has("cors") {
                        return Ok((&DeleteBucketCors as &'static dyn super::Operation, false));
                    }
                    if qs.has("encryption") {
                        return Ok((&DeleteBucketEncryption as &'static dyn super::Operation, false));
                    }
                    if qs.has("lifecycle") {
                        return Ok((&DeleteBucketLifecycle as &'static dyn super::Operation, false));
                    }
                    if qs.has("ownershipControls") {
                        return Ok((&DeleteBucketOwnershipControls as &'static dyn super::Operation, false));
                    }
                    if qs.has("policy") {
                        return Ok((&DeleteBucketPolicy as &'static dyn super::Operation, false));
                    }
                    if qs.has("replication") {
                        return Ok((&DeleteBucketReplication as &'static dyn super::Operation, false));
                    }
                    if qs.has("tagging") {
                        return Ok((&DeleteBucketTagging as &'static dyn super::Operation, false));
                    }
                    if qs.has("website") {
                        return Ok((&DeleteBucketWebsite as &'static dyn super::Operation, false));
                    }
                    if qs.has("publicAccessBlock") {
                        return Ok((&DeletePublicAccessBlock as &'static dyn super::Operation, false));
                    }
                }
                Ok((&DeleteBucket as &'static dyn super::Operation, false))
            }
            S3Path::Object { .. } => {
                if let Some(qs) = qs {
                    if qs.has("tagging") {
                        return Ok((&DeleteObjectTagging as &'static dyn super::Operation, false));
                    }
                }
                if let Some(qs) = qs {
                    if qs.has("uploadId") {
                        return Ok((&AbortMultipartUpload as &'static dyn super::Operation, false));
                    }
                }
                Ok((&DeleteObject as &'static dyn super::Operation, false))
            }
        },
        _ => Err(super::unknown_operation()),
    }
}
