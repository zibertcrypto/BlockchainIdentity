//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::True;

impl_payload! {
    /// Use this method to delete a sticker set that was created by the bot. Returns True on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub DeleteStickerSet (DeleteStickerSetSetters) => True {
        required {
            /// Sticker set name
            pub name: String [into],
        }
    }
}