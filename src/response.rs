// Copyright 2019 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT> or the Modified
// BSD license <LICENSE-BSD or https://opensource.org/licenses/BSD-3-Clause>,
// at your option. This file may not be copied, modified, or distributed
// except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use
// of the SAFE Network Software.

use crate::immutable_data::UnpubImmutableData;
use crate::mutable_data::{MutableData, SeqMutableData, UnseqMutableData, Value};
use crate::MessageId;
use routing::ClientError;
use std::collections::{BTreeMap, BTreeSet};

/// RPC responses from vaults.
#[derive(Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum Response<ErrorType> {
    GetUnpubIData(Result<UnpubImmutableData, ErrorType>),
    PutUnpubIData(Result<(), ErrorType>),
    DeleteUnpubIData(Result<(), ErrorType>),

    GetUnseqMData {
        res: Result<UnseqMutableData, ErrorType>,
        msg_id: MessageId,
    },
    PutUnseqMData {
        res: Result<(), ErrorType>,
        msg_id: MessageId,
    },
    GetSeqMData {
        res: Result<SeqMutableData, ErrorType>,
        msg_id: MessageId,
    },
    PutSeqMData {
        res: Result<(), ErrorType>,
        msg_id: MessageId,
    },
    GetMDataShell {
        res: Result<MutableData, ErrorType>,
        msg_id: MessageId,
    },
    GetMDataVersion {
        res: Result<u64, ErrorType>,
        msg_id: MessageId,
    },
    ListUnseqMDataEntries {
        res: Result<BTreeMap<Vec<u8>, Vec<u8>>, ErrorType>,
        msg_id: MessageId,
    },
    ListSeqMDataEntries {
        res: Result<BTreeMap<Vec<u8>, Value>, ErrorType>,
        msg_id: MessageId,
    },
    ListMDataKeys {
        res: Result<BTreeSet<Vec<u8>>, ErrorType>,
        msg_id: MessageId,
    },
    ListSeqMDataValues {
        res: Result<Vec<Value>, ErrorType>,
        msg_id: MessageId,
    },
    ListUnseqMDataValues {
        res: Result<Vec<Vec<u8>>, ErrorType>,
        msg_id: MessageId,
    },
}

use std::fmt;

impl fmt::Debug for Response<ClientError> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Response::GetUnpubIData { .. } => "Response::GetUnpubIData",
            Response::PutUnpubIData { .. } => "Response::PutUnpubIData",
            Response::DeleteUnpubIData { .. } => "Response::DeleteUnpubIData",
            Response::GetUnseqMData { .. } => "Response::GetUnseqMData",
            Response::PutUnseqMData { .. } => "Response::PutUnseqMData",
            Response::GetSeqMData { .. } => "Response::GetSeqMData",
            Response::PutSeqMData { .. } => "Response::PutSeqMData",
            Response::GetMDataShell { .. } => "Response::GetMDataShell",
            Response::GetMDataVersion { .. } => "Response::GetMDataVersion",
            Response::ListUnseqMDataEntries { .. } => "Response::ListUnseqMDataEntries",
            Response::ListSeqMDataEntries { .. } => "Response::ListSeqMDataEntries",
            Response::ListMDataKeys { .. } => "Response::ListMDataKeys",
            Response::ListSeqMDataValues { .. } => "Response::ListSeqMDataValues",
            Response::ListUnseqMDataValues { .. } => "Response::ListUnseqMDataValues",
        };
        write!(f, "{}", printable)
    }
}
