/*
    Cryptography utilities

    Copyright 2018 by Kzen Networks

    This file is part of Cryptography utilities library
    (https://github.com/KZen-networks/cryptography-utils)

    Cryptography utilities is free software: you can redistribute
    it and/or modify it under the terms of the GNU General Public
    License as published by the Free Software Foundation, either
    version 3 of the License, or (at your option) any later version.

    @license GPL-3.0+ <https://github.com/KZen-networks/cryptography-utils/blob/master/LICENSE>
*/

use BigInt;
use {FE, GE};

pub trait Hash {
    fn create_hash(big_ints: &[&BigInt]) -> BigInt;
    fn create_hash_from_ge(ge_vec: &[&GE]) -> FE;
}

pub trait KeyedHash {
    fn create_hmac(key: &BigInt, data: &[&BigInt]) -> BigInt;
}
