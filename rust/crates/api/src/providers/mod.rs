use std::future::Future;
use std::pin::Pin;

use crate::error::ApiError;
use crate::types::{MessageRequest, MessageResponse};

pub mod claw_provider;
pub mod openai_compat;

pub type ProviderFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, ApiError>> + Send + 'a>>;

pub trait Provider {
    type Stream;

    fn send_message<'a>(
        &'a self,
        request: &'a MessageRequest,
    ) -> ProviderFuture<'a, MessageResponse>;

    fn stream_message<'a>(
        &'a self,
        request: &'a MessageRequest,
    ) -> ProviderFuture<'a, Self::Stream>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderKind {
    ClawApi,
    Xai,
    OpenAi,
    MiniMax,
    DeepSeek,
    Qwen,
    Doubao,
    Glm,
    Gemini,
    /// 通用自定义 Provider，使用 CLAW_API_KEY + CLAW_BASE_URL
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProviderMetadata {
    pub provider: ProviderKind,
    pub auth_env: &'static str,
    pub base_url_env: &'static str,
    pub default_base_url: &'static str,
}

const MODEL_REGISTRY: &[(&str, ProviderMetadata)] = &[
    // ===== Claude (Anthropic) =====
    (
        "opus",
        ProviderMetadata {
            provider: ProviderKind::ClawApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: claw_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "sonnet",
        ProviderMetadata {
            provider: ProviderKind::ClawApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: claw_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "haiku",
        ProviderMetadata {
            provider: ProviderKind::ClawApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: claw_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "claude-opus-4-6",
        ProviderMetadata {
            provider: ProviderKind::ClawApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: claw_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "claude-sonnet-4-6",
        ProviderMetadata {
            provider: ProviderKind::ClawApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: claw_provider::DEFAULT_BASE_URL,
        },
    ),
    (
        "claude-haiku-4-5-20251213",
        ProviderMetadata {
            provider: ProviderKind::ClawApi,
            auth_env: "ANTHROPIC_API_KEY",
            base_url_env: "ANTHROPIC_BASE_URL",
            default_base_url: claw_provider::DEFAULT_BASE_URL,
        },
    ),
    // ===== xAI (Grok) =====
    (
        "grok",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-3",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-mini",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-3-mini",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    (
        "grok-2",
        ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        },
    ),
    // ===== MiniMax =====
    (
        "minimax",
        ProviderMetadata {
            provider: ProviderKind::MiniMax,
            auth_env: "MINIMAX_API_KEY",
            base_url_env: "MINIMAX_BASE_URL",
            default_base_url: openai_compat::DEFAULT_MINIMAX_BASE_URL,
        },
    ),
    // ===== DeepSeek =====
    (
        "deepseek",
        ProviderMetadata {
            provider: ProviderKind::DeepSeek,
            auth_env: "DEEPSEEK_API_KEY",
            base_url_env: "DEEPSEEK_BASE_URL",
            default_base_url: openai_compat::DEFAULT_DEEPSEEK_BASE_URL,
        },
    ),
    // ===== Qwen (千问) =====
    (
        "qwen",
        ProviderMetadata {
            provider: ProviderKind::Qwen,
            auth_env: "QWEN_API_KEY",
            base_url_env: "QWEN_BASE_URL",
            default_base_url: openai_compat::DEFAULT_QWEN_BASE_URL,
        },
    ),
    // ===== Doubao (豆包) =====
    (
        "doubao",
        ProviderMetadata {
            provider: ProviderKind::Doubao,
            auth_env: "DOUBAO_API_KEY",
            base_url_env: "DOUBAO_BASE_URL",
            default_base_url: openai_compat::DEFAULT_DOUBAO_BASE_URL,
        },
    ),
    // ===== GLM (智谱) =====
    (
        "glm",
        ProviderMetadata {
            provider: ProviderKind::Glm,
            auth_env: "GLM_API_KEY",
            base_url_env: "GLM_BASE_URL",
            default_base_url: openai_compat::DEFAULT_GLM_BASE_URL,
        },
    ),
    // ===== Gemini =====
    (
        "gemini",
        ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_GEMINI_BASE_URL,
        },
    ),
];

#[must_use]
pub fn resolve_model_alias(model: &str) -> String {
    let trimmed = model.trim();
    let lower = trimmed.to_ascii_lowercase();
    MODEL_REGISTRY
        .iter()
        .find_map(|(alias, metadata)| {
            (*alias == lower).then_some(match metadata.provider {
                ProviderKind::ClawApi => match *alias {
                    "opus" => "claude-opus-4-6",
                    "sonnet" => "claude-sonnet-4-6",
                    "haiku" => "claude-haiku-4-5-20251213",
                    _ => trimmed,
                },
                ProviderKind::Xai => match *alias {
                    "grok" | "grok-3" => "grok-3",
                    "grok-mini" | "grok-3-mini" => "grok-3-mini",
                    "grok-2" => "grok-2",
                    _ => trimmed,
                },
                // 国产/自定义模型：别名直接透传原始模型名
                _ => trimmed,
            })
        })
        .map_or_else(|| trimmed.to_string(), ToOwned::to_owned)
}

#[must_use]
pub fn metadata_for_model(model: &str) -> Option<ProviderMetadata> {
    let canonical = resolve_model_alias(model);
    let lower = canonical.to_ascii_lowercase();
    // 精确匹配注册表
    if let Some((_, metadata)) = MODEL_REGISTRY.iter().find(|(alias, _)| *alias == lower) {
        return Some(*metadata);
    }
    // 前缀匹配：根据模型名称前缀自动推断 Provider
    if lower.starts_with("grok") {
        return Some(ProviderMetadata {
            provider: ProviderKind::Xai,
            auth_env: "XAI_API_KEY",
            base_url_env: "XAI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_XAI_BASE_URL,
        });
    }
    if lower.starts_with("minimax") || lower.starts_with("abab") {
        return Some(ProviderMetadata {
            provider: ProviderKind::MiniMax,
            auth_env: "MINIMAX_API_KEY",
            base_url_env: "MINIMAX_BASE_URL",
            default_base_url: openai_compat::DEFAULT_MINIMAX_BASE_URL,
        });
    }
    if lower.starts_with("deepseek") {
        return Some(ProviderMetadata {
            provider: ProviderKind::DeepSeek,
            auth_env: "DEEPSEEK_API_KEY",
            base_url_env: "DEEPSEEK_BASE_URL",
            default_base_url: openai_compat::DEFAULT_DEEPSEEK_BASE_URL,
        });
    }
    if lower.starts_with("qwen") {
        return Some(ProviderMetadata {
            provider: ProviderKind::Qwen,
            auth_env: "QWEN_API_KEY",
            base_url_env: "QWEN_BASE_URL",
            default_base_url: openai_compat::DEFAULT_QWEN_BASE_URL,
        });
    }
    if lower.starts_with("doubao") || lower.starts_with("ep-") {
        return Some(ProviderMetadata {
            provider: ProviderKind::Doubao,
            auth_env: "DOUBAO_API_KEY",
            base_url_env: "DOUBAO_BASE_URL",
            default_base_url: openai_compat::DEFAULT_DOUBAO_BASE_URL,
        });
    }
    if lower.starts_with("glm") {
        return Some(ProviderMetadata {
            provider: ProviderKind::Glm,
            auth_env: "GLM_API_KEY",
            base_url_env: "GLM_BASE_URL",
            default_base_url: openai_compat::DEFAULT_GLM_BASE_URL,
        });
    }
    if lower.starts_with("gemini") {
        return Some(ProviderMetadata {
            provider: ProviderKind::Gemini,
            auth_env: "GEMINI_API_KEY",
            base_url_env: "GEMINI_BASE_URL",
            default_base_url: openai_compat::DEFAULT_GEMINI_BASE_URL,
        });
    }
    None
}

#[must_use]
pub fn detect_provider_kind(model: &str) -> ProviderKind {
    // 1. 通用自定义 Provider（CLAW_API_KEY + CLAW_BASE_URL 优先级最高）
    if openai_compat::has_api_key("CLAW_API_KEY") {
        return ProviderKind::Custom;
    }
    // 2. 根据模型名称匹配已知 Provider
    if let Some(metadata) = metadata_for_model(model) {
        return metadata.provider;
    }
    // 3. 依次检测各 Provider 的环境变量
    if claw_provider::has_auth_from_env_or_saved().unwrap_or(false) {
        return ProviderKind::ClawApi;
    }
    if openai_compat::has_api_key("MINIMAX_API_KEY") {
        return ProviderKind::MiniMax;
    }
    if openai_compat::has_api_key("DEEPSEEK_API_KEY") {
        return ProviderKind::DeepSeek;
    }
    if openai_compat::has_api_key("QWEN_API_KEY") {
        return ProviderKind::Qwen;
    }
    if openai_compat::has_api_key("DOUBAO_API_KEY") {
        return ProviderKind::Doubao;
    }
    if openai_compat::has_api_key("GLM_API_KEY") {
        return ProviderKind::Glm;
    }
    if openai_compat::has_api_key("GEMINI_API_KEY") {
        return ProviderKind::Gemini;
    }
    if openai_compat::has_api_key("OPENAI_API_KEY") {
        return ProviderKind::OpenAi;
    }
    if openai_compat::has_api_key("XAI_API_KEY") {
        return ProviderKind::Xai;
    }
    // 最终回退到 Custom
    ProviderKind::Custom
}

/// 根据 ProviderKind 获取对应的 OpenAiCompatConfig
#[must_use]
pub fn config_for_provider(kind: ProviderKind) -> openai_compat::OpenAiCompatConfig {
    match kind {
        ProviderKind::Xai => openai_compat::OpenAiCompatConfig::xai(),
        ProviderKind::OpenAi => openai_compat::OpenAiCompatConfig::openai(),
        ProviderKind::MiniMax => openai_compat::OpenAiCompatConfig::minimax(),
        ProviderKind::DeepSeek => openai_compat::OpenAiCompatConfig::deepseek(),
        ProviderKind::Qwen => openai_compat::OpenAiCompatConfig::qwen(),
        ProviderKind::Doubao => openai_compat::OpenAiCompatConfig::doubao(),
        ProviderKind::Glm => openai_compat::OpenAiCompatConfig::glm(),
        ProviderKind::Gemini => openai_compat::OpenAiCompatConfig::gemini(),
        ProviderKind::Custom | ProviderKind::ClawApi => openai_compat::OpenAiCompatConfig::custom(),
    }
}

#[must_use]
pub fn max_tokens_for_model(model: &str) -> u32 {
    let canonical = resolve_model_alias(model);
    if canonical.contains("opus") {
        32_000
    } else {
        64_000
    }
}

#[cfg(test)]
mod tests {
    use super::{detect_provider_kind, max_tokens_for_model, resolve_model_alias, ProviderKind};

    #[test]
    fn resolves_grok_aliases() {
        assert_eq!(resolve_model_alias("grok"), "grok-3");
        assert_eq!(resolve_model_alias("grok-mini"), "grok-3-mini");
        assert_eq!(resolve_model_alias("grok-2"), "grok-2");
    }

    #[test]
    fn detects_provider_from_model_name_first() {
        // 注意：如果设置了 CLAW_API_KEY，Custom 优先级最高
        // 这里只测试 model 前缀匹配
        assert_eq!(
            detect_provider_kind("claude-sonnet-4-6"),
            if std::env::var("CLAW_API_KEY").is_ok() {
                ProviderKind::Custom
            } else {
                ProviderKind::ClawApi
            }
        );
    }

    #[test]
    fn keeps_existing_max_token_heuristic() {
        assert_eq!(max_tokens_for_model("opus"), 32_000);
        assert_eq!(max_tokens_for_model("grok-3"), 64_000);
    }

    #[test]
    fn detects_minimax_from_model_name() {
        assert_eq!(
            super::metadata_for_model("MiniMax-M2.7")
                .map(|m| m.provider),
            Some(ProviderKind::MiniMax)
        );
    }

    #[test]
    fn detects_deepseek_from_model_name() {
        assert_eq!(
            super::metadata_for_model("deepseek-chat")
                .map(|m| m.provider),
            Some(ProviderKind::DeepSeek)
        );
    }
}
