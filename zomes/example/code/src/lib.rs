#![feature(slice_patterns)]
#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

extern crate hdk_proc_macros;
use hdk_proc_macros::zome;

use hdk::{
    AGENT_ADDRESS, AGENT_ID_STR, DNA_ADDRESS, DNA_NAME,
    entry_definition::ValidatingEntryType,
    error::{ZomeApiResult, ZomeApiError},
    holochain_core_types::{
        agent::AgentId,
        dna::entry_types::Sharing,
        entry::Entry,
        time::{Period, Timeout},
    },
    holochain_json_api::{
        error::JsonError,
        json::JsonString,
    },
    holochain_persistence_api::{
        cas::content::Address,
    },
};

use std::convert::TryInto;


// see https://developer.holochain.org/api/{{ version }}/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MyEntry {
    content: String,
}


#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq)]
pub struct WhoamiResult {
    pub hdk_version:	String,
    pub hdk_hash:	String,
    pub dna_address:	String,
    pub dna_name:	String,
    pub agent_id:	AgentId,
    pub agent_address:	String,
}

/// whoami_internal -- Return details of the local Agent or (if specified) some other Agent
pub fn whoami_internal() -> ZomeApiResult<WhoamiResult> {
    Ok(WhoamiResult {
        hdk_version: hdk::version()?,
        hdk_hash: hdk::version_hash()?,
        dna_name: DNA_NAME.to_string(),
        dna_address: DNA_ADDRESS.to_string(),
        agent_id: JsonString::from_json(&AGENT_ID_STR).try_into()?,
        agent_address: AGENT_ADDRESS.to_string(),
    })
}


#[derive(Serialize, Deserialize, Debug, DefaultJson, PartialEq)]
pub enum Message {
    None,
    Ping(String),
    Pong((Address, String))
}

/// receive_responder -- returns the supplied arbitrary JSON-encoded payload via a JSON-encoded ZomeApiResult
pub fn receive_responder(
    from: Address,
    payload: JsonString
) -> JsonString {
    // Attempt to parse the provide JSON as the expected payload Message type, returning a
    // valid-looking JSON ZomeApiResult on Error (we could do this in various more direct ways...)
    //println!("receive_responder: received: from: {:?}, payload: {:?}", &from, &payload);
    let message: Message = match payload.try_into() {
        Ok(message) => message,
        Err(e) =>
            return json!({
                "Err": {
                    "HolochainError": {
                        "message": format!(
                            "receive_responder: Failed to parse JSON payload as Message: {}",
                            e )
                    }
                }
            }).into(),
    };

    // Formulate the response Ok/Err ZomeApiResult, convert to JSON
    let response = JsonString::from(responder(from, message));
    //println!("receive_responder: response: {:?}", &response);
    response
}

/// responder -- Process the decoded inter-agent Message, yielding Ok/Err Result Message
pub fn responder(
    from: Address,
    message: Message
) -> ZomeApiResult<Message> {
    //println!("responder: message: {:?}", &message);
    Ok(match message {
        Message::None => Message::None,
        Message::Ping(ping) => Message::Pong((from, ping)),
        Message::Pong(pong) => return Err(ZomeApiError::Internal(
            format!("Unhandled: {:?}", Message::Pong(pong))
        )),
    })
}


#[zome]
pub mod example {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[receive]
    fn receive(from: Address, payload_json: String) {
        //println!("receive: from: {:?}: {:?}", &from, &payload_json);
        let response_json = receive_responder(from, JsonString::from_json(&payload_json)).to_string();
        //println!("receive: response: {:?}", &response_json);
        response_json
    }

    #[entry_def]
    fn my_entry_definition() -> ValidatingEntryType {
        entry!(
            name: "my_entry",
            description: "this is my entry definition",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },

            validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
                Ok(())
            }
        )
    }

    #[zome_fn("hc_public")]
    fn whoami() -> ZomeApiResult<WhoamiResult> {
        whoami_internal()
    }

    #[zome_fn("hc_public")]
    fn create_my_entry(
        entry: MyEntry
    ) -> ZomeApiResult<Address> {
        let entry = Entry::App("my_entry".into(), entry.into());
        let address = hdk::commit_entry(&entry)?;
        Ok(address)
    }

    #[zome_fn("hc_public")]
    fn get_my_entry(
        address: Address
    ) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }

    /// send_message -- Carries an arbitrary Message type via JSON Strings to/from counterparty Agent
    #[zome_fn("hc_public")]
    fn send_message(
        to: Address,
        message: Message,
        timeout: Option<Period> // eg. "2500ms", "1m30s"
    ) -> ZomeApiResult<Message> {
        // Encode the Message to a JSON-encoded String
        let timeout = match timeout {
            Some(period) => period.into(),
            None => Timeout::from(5000),
        };
        hdk::debug(format!("send_message: message: {:?}", &message)).ok();
        let message_json: String = JsonString::from( message ).to_string();
        hdk::debug(format!("send_message: message_json: {:?}", &message_json)).ok();

        // Raw agent-agent communication occurs as Strings, with the designated timeout
        let reply_json: String = hdk::send( to, message_json, timeout )?;

        // Decode the JSON-encoded ZomeApiResult-wrapped reply Message
        hdk::debug(format!("send_message: reply_json: {:?}", &reply_json)).ok();
        let reply = JsonString::from_json( &reply_json );
        let result: ZomeApiResult<Message> = reply.try_into()?;
        hdk::debug(format!("send_message: result: {:?}", &result)).ok();
        result
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    use hdk::{
        holochain_core_types::{
            error::{
                RibosomeEncodedValue, RibosomeEncodingBits,
            },
        },
    };

    // See .../holochain-rust/crates/core/src/nucleus/ribosome/api/mod.rs for definitive list
    #[no_mangle]
    pub fn hc_debug(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_commit_entry(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_get_entry(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_update_entry(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_remove_entry(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_init_globals(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_call(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_link_entries(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_get_links(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_get_links_count(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_query(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_entry_address(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_send(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_sleep(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_remove_link(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_crypto(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_sign_one_time(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_verify_signature(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_keystore_list(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_keystore_new_random(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_keystore_derive_seed(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_keystore_derive_key(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_keystore_sign(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_keystore_get_public_key(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_commit_capability_grant(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_commit_capability_claim(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_emit_signal(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[no_mangle]
    pub fn hc_meta(_: RibosomeEncodingBits) -> RibosomeEncodingBits {
        RibosomeEncodedValue::Success.into()
    }

    #[test]
    fn smoke() {
        // Lets confirm transport of some arbitrary JSON carried via send/receive 
        let body = JsonString::from( "My hovercraft is full of eels!" );
        let address = Address::from( "HcScjwO9ji9633ZYxa6IYubHJHW6ctfoufv5eq4F7ZOxay8wR76FP4xeG9pY3ui" );
        let payload_json = JsonString::from(Message::Ping(body.to_string()));

        let response = receive_responder( address.clone(), payload_json );

        //println!( "response: {:?}", &response );
        let received_maybe: Result<ZomeApiResult<Message>, _> = response.try_into();
        assert!( received_maybe.is_ok() );
        if received_maybe.is_ok() {
            let received = received_maybe.unwrap();
            //println!( "received: {:?}", &received );
            assert!( received.is_ok() );
            if received.is_ok() {
                // And lets confirm that our Ping round-tripped
                assert_eq!( received.unwrap(), Message::Pong((address,body.to_string())));
            }
        }
    }

}
