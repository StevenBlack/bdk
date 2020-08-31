// Magical Bitcoin Library
// Written in 2020 by
//     Alekos Filini <alekos.filini@gmail.com>
//
// Copyright (c) 2020 Magical Bitcoin
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::convert::AsRef;

use bitcoin::blockdata::transaction::{OutPoint, Transaction, TxOut};
use bitcoin::hash_types::Txid;

use serde::{Deserialize, Serialize};

// TODO serde flatten?
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptType {
    External = 0,
    Internal = 1,
}

impl ScriptType {
    pub fn as_byte(&self) -> u8 {
        match self {
            ScriptType::External => 'e' as u8,
            ScriptType::Internal => 'i' as u8,
        }
    }

    pub fn is_internal(&self) -> bool {
        self == &ScriptType::Internal
    }
}

impl AsRef<[u8]> for ScriptType {
    fn as_ref(&self) -> &[u8] {
        match self {
            ScriptType::External => b"e",
            ScriptType::Internal => b"i",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UTXO {
    pub outpoint: OutPoint,
    pub txout: TxOut,
    pub is_internal: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct TransactionDetails {
    pub transaction: Option<Transaction>,
    pub txid: Txid,
    pub timestamp: u64,
    pub received: u64,
    pub sent: u64,
    pub fees: u64,
    pub height: Option<u32>,
}
