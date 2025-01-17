use crate::command::args::{
    Arg, ArgumentConsumer, DefaultNameArgConsumer, FindArg, GetClientSideArgParser,
};
use crate::command::dispatcher::CommandError;
use crate::command::tree::RawArgs;
use crate::command::CommandSender;
use crate::server::Server;
use async_trait::async_trait;
use pumpkin_protocol::client::play::{
    CommandSuggestion, ProtoCmdArgParser, ProtoCmdArgSuggestionType,
};

pub(crate) struct ResourceLocationArgumentConsumer {
    autocomplete: bool,
}

impl GetClientSideArgParser for ResourceLocationArgumentConsumer {
    fn get_client_side_parser(&self) -> ProtoCmdArgParser {
        ProtoCmdArgParser::ResourceLocation
    }

    fn get_client_side_suggestion_type_override(&self) -> Option<ProtoCmdArgSuggestionType> {
        Some(ProtoCmdArgSuggestionType::AskServer)
    }
}

#[async_trait]
impl ArgumentConsumer for ResourceLocationArgumentConsumer {
    async fn consume<'a>(
        &self,
        _sender: &CommandSender<'a>,
        _server: &'a Server,
        args: &mut RawArgs<'a>,
    ) -> Option<Arg<'a>> {
        let s = args.pop()?;

        let name = if s.contains(':') {
            s.to_string()
        } else {
            format!("minecraft:{s}")
        };

        Some(Arg::ResourceLocation(name))
    }

    async fn suggest<'a>(
        &self,
        _sender: &CommandSender<'a>,
        _server: &'a Server,
        _input: &'a str,
    ) -> Result<Option<Vec<CommandSuggestion<'a>>>, CommandError> {
        if !self.autocomplete {
            return Ok(None);
        }

        let suggestions = _server
            .bossbars
            .lock()
            .await
            .custom_bossbars
            .keys()
            .map(|suggestion| CommandSuggestion::new(suggestion.clone(), None))
            .collect();

        Ok(Some(suggestions))
    }
}

impl DefaultNameArgConsumer for ResourceLocationArgumentConsumer {
    fn default_name(&self) -> &'static str {
        "id"
    }

    fn get_argument_consumer(&self) -> &dyn ArgumentConsumer {
        self
    }
}

impl<'a> FindArg<'a> for ResourceLocationArgumentConsumer {
    type Data = &'a str;

    fn find_arg(args: &'a super::ConsumedArgs, name: &'a str) -> Result<Self::Data, CommandError> {
        match args.get(name) {
            Some(Arg::ResourceLocation(data)) => Ok(data),
            _ => Err(CommandError::InvalidConsumption(Some(name.to_string()))),
        }
    }
}

impl ResourceLocationArgumentConsumer {
    pub(crate) const fn new(autocomplete: bool) -> Self {
        Self { autocomplete }
    }
}
