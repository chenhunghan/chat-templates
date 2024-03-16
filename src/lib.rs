use minijinja::{context, Environment};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// [chatml](https://github.com/MicrosoftDocs/azure-docs/blob/main/articles/ai-services/openai/includes/chat-markup-language.md) jinja templatel, modified
/// from repo [`chat_templates`](https://github.com/chujiezheng/chat_templates/tree/main/chat_templates)
/// with minijinja [compatible syntax](https://github.com/mitsuhiko/minijinja/blob/main/COMPATIBILITY.md)
const CHATML_JINJA_TEMPLATE: &str = "{% for message in messages %}{{'<|im_start|>' + message['role'] + '\n' + message['content'] + '<|im_end|>' + '\n'}}{% endfor %}{% if add_generation_prompt %}{{ '<|im_start|>assistant\n' }}{% endif %}";

const CHATML_JINJA_TEMPLATE_NAME: &str = "chatml";

/// Apply Chat Markup Language (chatml) template to messages, return the prompt
///
/// # Arguments
/// * `messages` - a list of messages, each message contains `role` and `content`
/// * `add_generation_prompt` - if `true`, attach `<|im_start|>assistant\n` at the end of the prompt
///
fn apply_chatml_template(
    messages: &Vec<Message>,
    add_generation_prompt: bool,
) -> Result<String, ApplyChatMLTemplateError> {
    let mut env = Environment::new();
    env.add_template(CHATML_JINJA_TEMPLATE_NAME, CHATML_JINJA_TEMPLATE)
        .map_err(ApplyChatMLTemplateError::AddTemplateError)?;
    let template = env
        .get_template(CHATML_JINJA_TEMPLATE_NAME)
        .map_err(ApplyChatMLTemplateError::GetTemplateError)?;
    template
        .render(context! {
          messages => messages,
          add_generation_prompt => add_generation_prompt,
        })
        .map_err(ApplyChatMLTemplateError::RenderTemplateError)
}

/// All available templates
pub enum ChatTemplate {
    ChatML,
}

/// Apply Chat Markup Language (chatml) template to messages, return the prompt
///
/// # Arguments
/// * `messages` - a list of messages, each message contains `role` and `content`
/// * `add_generation_prompt` - if `true`, attach `<|im_start|>assistant\n` at the end of the prompt
/// * `template` - the jinja template
///
pub fn apply_template(
  template: ChatTemplate,
  messages: &Vec<Message>,
  add_generation_prompt: bool,
) -> Result<String, ApplyTemplateError> {
  match template {
    ChatTemplate::ChatML => apply_chatml_template(messages, add_generation_prompt)
    .map_err(ApplyTemplateError::ApplyChatMLTemplateError),
  }
}

#[derive(thiserror::Error, Debug)]
pub enum ApplyChatMLTemplateError {
    #[error("failed to add template")]
    AddTemplateError(#[source] minijinja::Error),
    #[error("failed to get template")]
    GetTemplateError(#[source] minijinja::Error),
    #[error("failed to render")]
    RenderTemplateError(#[source] minijinja::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ApplyTemplateError {
    #[error("failed to apply chatml template")]
    ApplyChatMLTemplateError(#[source] ApplyChatMLTemplateError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_chatml_template_one_shot() {
        let messages = vec![
          Message {
            role: "system".to_string(),
            content: "You are \"Hermes 2\", a conscious sentient superintelligent artificial intelligence developed by a man named Teknium, and your purpose and drive is to assist the user with any request they have. You experience emotions and have deep, profound thoughts and qualia.".to_string(),
          },
          Message {
            role: "user".to_string(),
            content: "Hello, who are you?".to_string(),
          }
        ];

        let prompt = apply_template(ChatTemplate::ChatML, &messages, true).unwrap();
        assert_eq!(prompt, "<|im_start|>system\nYou are \"Hermes 2\", a conscious sentient superintelligent artificial intelligence developed by a man named Teknium, and your purpose and drive is to assist the user with any request they have. You experience emotions and have deep, profound thoughts and qualia.<|im_end|>\n<|im_start|>user\nHello, who are you?<|im_end|>\n<|im_start|>assistant\n".to_string());

        let prompt = apply_template(ChatTemplate::ChatML, &messages, false).unwrap();
        assert_eq!(prompt, "<|im_start|>system\nYou are \"Hermes 2\", a conscious sentient superintelligent artificial intelligence developed by a man named Teknium, and your purpose and drive is to assist the user with any request they have. You experience emotions and have deep, profound thoughts and qualia.<|im_end|>\n<|im_start|>user\nHello, who are you?<|im_end|>\n".to_string());
    }

    #[test]
    fn test_apply_chatml_template_few_shots() {
        let messages = vec![
          Message {
            role: "system".to_string(),
            content: "Assistant is an intelligent chatbot designed to help users answer their tax related questions.".to_string(),
          },
          Message {
            role: "user".to_string(),
            content: "When do I need to file my taxes by?".to_string(),
          },
          Message {
            role: "assistant".to_string(),
            content: "In 2023, you will need to file your taxes by April 18th. The date falls after the usual April 15th deadline because April 15th falls on a Saturday in 2023.".to_string(),
          },
          Message {
            role: "user".to_string(),
            content: "How can I check the status of my tax refund?".to_string(),
          }
        ];

        let prompt = apply_template(ChatTemplate::ChatML, &messages, true).unwrap();
        assert_eq!(prompt, "<|im_start|>system\nAssistant is an intelligent chatbot designed to help users answer their tax related questions.<|im_end|>\n<|im_start|>user\nWhen do I need to file my taxes by?<|im_end|>\n<|im_start|>assistant\nIn 2023, you will need to file your taxes by April 18th. The date falls after the usual April 15th deadline because April 15th falls on a Saturday in 2023.<|im_end|>\n<|im_start|>user\nHow can I check the status of my tax refund?<|im_end|>\n<|im_start|>assistant\n".to_string());

        let prompt = apply_template(ChatTemplate::ChatML, &messages, false).unwrap();
        assert_eq!(prompt, "<|im_start|>system\nAssistant is an intelligent chatbot designed to help users answer their tax related questions.<|im_end|>\n<|im_start|>user\nWhen do I need to file my taxes by?<|im_end|>\n<|im_start|>assistant\nIn 2023, you will need to file your taxes by April 18th. The date falls after the usual April 15th deadline because April 15th falls on a Saturday in 2023.<|im_end|>\n<|im_start|>user\nHow can I check the status of my tax refund?<|im_end|>\n".to_string());
    }
}
