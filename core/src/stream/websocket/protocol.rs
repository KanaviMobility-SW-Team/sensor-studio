//! 커스텀 웹소켓 제어 프로토콜 규격 모듈

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::runtime::adapter::EngineExtensionApiInfo;

/// 클라이언트 전송 웹소켓 제어 명령어
#[derive(Debug, Deserialize)]
#[serde(tag = "op")]
pub enum ClientControlMessage {
    #[serde(rename = "getEngineApis")]
    GetEngineApis {
        #[serde(rename = "instanceId")]
        instance_id: String,
    },

    #[serde(rename = "callEngineApi")]
    CallEngineApi {
        #[serde(rename = "instanceId")]
        instance_id: String,
        #[serde(rename = "apiName")]
        api_name: String,
        input: Value,
    },
}

/// 서버 송신 제어 상태 및 응답 결과
#[derive(Debug, Serialize)]
#[serde(tag = "op")]
pub enum ServerControlMessage {
    #[serde(rename = "engineApis")]
    EngineApis {
        #[serde(rename = "instanceId")]
        instance_id: String,
        apis: Vec<EngineExtensionApiInfoDto>,
    },

    #[serde(rename = "engineApiResult")]
    EngineApiResult {
        #[serde(rename = "instanceId")]
        instance_id: String,
        #[serde(rename = "apiName")]
        api_name: String,
        output: Value,
    },

    #[serde(rename = "engineApiError")]
    EngineApiError {
        #[serde(rename = "instanceId")]
        instance_id: String,
        #[serde(rename = "apiName")]
        api_name: Option<String>,
        message: String,
    },
}

/// 엔진 확장 API 메타데이터 전달 객체
#[derive(Debug, Serialize)]
pub struct EngineExtensionApiInfoDto {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchemaJson")]
    pub input_schema_json: String,
    #[serde(rename = "outputSchemaJson")]
    pub output_schema_json: String,
}

impl From<EngineExtensionApiInfo> for EngineExtensionApiInfoDto {
    fn from(value: EngineExtensionApiInfo) -> Self {
        Self {
            name: value.name,
            description: value.description,
            input_schema_json: value.input_schema_json,
            output_schema_json: value.output_schema_json,
        }
    }
}
