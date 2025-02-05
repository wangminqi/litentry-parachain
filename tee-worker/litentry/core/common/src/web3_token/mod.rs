// Copyright 2020-2024 Trust Computing GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry. If not, see <https://www.gnu.org/licenses/>.

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use std::{vec, vec::Vec};

use litentry_primitives::Web3TokenType;

use crate::Web3Network;

pub trait TokenName {
	fn get_token_name(&self) -> &'static str;
}

impl TokenName for Web3TokenType {
	fn get_token_name(&self) -> &'static str {
		match self {
			Self::Bnb => "BNB",
			Self::Eth => "ETH",
			Self::SpaceId => "SPACE_ID",
			Self::Lit => "LIT",
			Self::Wbtc => "WBTC",
			Self::Usdc => "USDC",
			Self::Usdt => "USDT",
			Self::Crv => "CRV",
			Self::Matic => "MATIC",
			Self::Dydx => "DYDX",
			Self::Amp => "AMP",
			Self::Cvx => "CVX",
			Self::Tusd => "TUSD",
			Self::Usdd => "USDD",
			Self::Gusd => "GUSD",
			Self::Link => "LINK",
			Self::Grt => "GRT",
			Self::Comp => "COMP",
			Self::People => "PEOPLE",
			Self::Gtc => "GTC",
			Self::Ton => "TON",
			Self::Trx => "TRX",
			Self::Nfp => "NFP",
			Self::Sol => "SOL",
			Self::Mcrt => "MCRT",
			Self::Btc => "BTC",
		}
	}
}

pub trait TokenAddress {
	fn get_token_address(&self, network: Web3Network) -> Option<&'static str>;
}

impl TokenAddress for Web3TokenType {
	fn get_token_address(&self, network: Web3Network) -> Option<&'static str> {
		match (self, network) {
			// Bnb
			(Self::Bnb, Web3Network::Ethereum) =>
				Some("0xb8c77482e45f1f44de1745f52c74426c631bdd52"),
			// Eth
			(Self::Eth, Web3Network::Bsc) => Some("0x2170ed0880ac9a755fd29b2688956bd959f933f8"),
			// SpaceId
			(Self::SpaceId, Web3Network::Bsc) | (Self::SpaceId, Web3Network::Ethereum) =>
				Some("0x2dff88a56767223a5529ea5960da7a3f5f766406"),
			// Lit
			(Self::Lit, Web3Network::Bsc) | (Self::Lit, Web3Network::Ethereum) =>
				Some("0xb59490ab09a0f526cc7305822ac65f2ab12f9723"),
			// Wbtc
			(Self::Wbtc, Web3Network::Ethereum) =>
				Some("0x2260fac5e5542a773aa44fbcfedf7c193bc2c599"),
			// Usdc
			(Self::Usdc, Web3Network::Bsc) => Some("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d"),
			(Self::Usdc, Web3Network::Ethereum) =>
				Some("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"),
			// Usdt
			(Self::Usdt, Web3Network::Bsc) => Some("0x55d398326f99059ff775485246999027b3197955"),
			(Self::Usdt, Web3Network::Ethereum) =>
				Some("0xdac17f958d2ee523a2206206994597c13d831ec7"),
			// Crv
			(Self::Crv, Web3Network::Ethereum) =>
				Some("0xd533a949740bb3306d119cc777fa900ba034cd52"),
			// Matic
			(Self::Matic, Web3Network::Bsc) => Some("0xcc42724c6683b7e57334c4e856f4c9965ed682bd"),
			(Self::Matic, Web3Network::Ethereum) =>
				Some("0x7d1afa7b718fb893db30a3abc0cfc608aacfebb0"),
			// Dydx
			(Self::Dydx, Web3Network::Ethereum) =>
				Some("0x92d6c1e31e14520e676a687f0a93788b716beff5"),
			// Amp
			(Self::Amp, Web3Network::Ethereum) =>
				Some("0xff20817765cb7f73d4bde2e66e067e58d11095c2"),
			// Cvx
			(Self::Cvx, Web3Network::Ethereum) =>
				Some("0x4e3fbd56cd56c3e72c1403e103b45db9da5b9d2b"),
			// Tusd
			(Self::Tusd, Web3Network::Bsc) => Some("0x40af3827F39D0EAcBF4A168f8D4ee67c121D11c9"),
			(Self::Tusd, Web3Network::Ethereum) =>
				Some("0x0000000000085d4780b73119b644ae5ecd22b376"),
			// Usdd
			(Self::Usdd, Web3Network::Bsc) => Some("0xd17479997f34dd9156deef8f95a52d81d265be9c"),
			(Self::Usdd, Web3Network::Ethereum) =>
				Some("0x0c10bf8fcb7bf5412187a595ab97a3609160b5c6"),
			// Gusd
			(Self::Gusd, Web3Network::Ethereum) =>
				Some("0x056fd409e1d7a124bd7017459dfea2f387b6d5cd"),
			// Link
			(Self::Link, Web3Network::Bsc) => Some("0xf8a0bf9cf54bb92f17374d9e9a321e6a111a51bd"),
			(Self::Link, Web3Network::Ethereum) =>
				Some("0x514910771af9ca656af840dff83e8264ecf986ca"),
			// Grt
			(Self::Grt, Web3Network::Bsc) => Some("0x52ce071bd9b1c4b00a0b92d298c512478cad67e8"),
			(Self::Grt, Web3Network::Ethereum) =>
				Some("0xc944e90c64b2c07662a292be6244bdf05cda44a7"),
			// Comp
			(Self::Comp, Web3Network::Ethereum) =>
				Some("0xc00e94cb662c3520282e6f5717214004a7f26888"),
			// People
			(Self::People, Web3Network::Ethereum) =>
				Some("0x7a58c0be72be218b41c608b7fe7c5bb630736c71"),
			// Gtc
			(Self::Gtc, Web3Network::Ethereum) =>
				Some("0xde30da39c46104798bb5aa3fe8b9e0e1f348163f"),
			// Ton
			(Self::Ton, Web3Network::Bsc) => Some("0x76a797a59ba2c17726896976b7b3747bfd1d220f"),
			(Self::Ton, Web3Network::Ethereum) =>
				Some("0x582d872a1b094fc48f5de31d3b73f2d9be47def1"),
			// Trx
			(Self::Trx, Web3Network::Bsc) => Some("0xCE7de646e7208a4Ef112cb6ed5038FA6cC6b12e3"),
			(Self::Trx, Web3Network::Ethereum) =>
				Some("0x50327c6c5a14dcade707abad2e27eb517df87ab5"),
			// Nfp
			(Self::Nfp, Web3Network::Bsc) => Some("0x75e8ddb518bb757b4282cd5b83bb70d4101d12fb"),
			// Sol
			(Self::Sol, Web3Network::Bsc) => Some("0x570a5d26f7765ecb712c0924e4de545b89fd43df"),
			(Self::Sol, Web3Network::Ethereum) =>
				Some("0x5288738df1aeb0894713de903e1d0c001eeb7644"),
			// Mcrt
			(Self::Mcrt, Web3Network::Bsc) => Some("0x4b8285aB433D8f69CB48d5Ad62b415ed1a221e4f"),
			(Self::Mcrt, Web3Network::Ethereum) =>
				Some("0xde16ce60804a881e9f8c4ebb3824646edecd478d"),
			(Self::Mcrt, Web3Network::Solana) =>
				Some("FADm4QuSUF1K526LvTjvbJjKzeeipP6bj5bSzp3r6ipq"),
			_ => None,
		}
	}
}

pub trait TokenDecimals {
	fn get_decimals(&self, network: Web3Network) -> u64;
}

impl TokenDecimals for Web3TokenType {
	fn get_decimals(&self, network: Web3Network) -> u64 {
		let decimals = match (self, network) {
			// Bnb
			(Self::Bnb, Web3Network::Bsc) | (Self::Bnb, Web3Network::Ethereum) |
			// Eth
			(Self::Eth, Web3Network::Bsc) | (Self::Eth, Web3Network::Ethereum) |
			// SpaceId
			(Self::SpaceId, Web3Network::Bsc) | (Self::SpaceId, Web3Network::Ethereum) |
			// Lit
			(Self::Lit, Web3Network::Bsc) | (Self::Lit, Web3Network::Ethereum) |
			// Usdc
			(Self::Usdc, Web3Network::Bsc) |
			// Usdt
			(Self::Usdt, Web3Network::Bsc) |
			// Crv
			(Self::Crv, Web3Network::Ethereum) |
			// Matic
			(Self::Matic, Web3Network::Bsc) | (Self::Matic, Web3Network::Ethereum) |
			// Dydx
			(Self::Dydx, Web3Network::Ethereum) |
			// Amp
			(Self::Amp, Web3Network::Ethereum) |
			// Cvx
			(Self::Cvx, Web3Network::Ethereum) |
			// Tusd
			(Self::Tusd, Web3Network::Bsc) | (Self::Tusd, Web3Network::Ethereum) |
			// Usdd
			(Self::Usdd, Web3Network::Bsc) | (Self::Usdd, Web3Network::Ethereum) |
			// Link
			(Self::Link, Web3Network::Bsc) | (Self::Link, Web3Network::Ethereum) |
			// Grt
			(Self::Grt, Web3Network::Bsc) | (Self::Grt, Web3Network::Ethereum) |
			// Comp
			(Self::Comp, Web3Network::Ethereum) |
			// People
			(Self::People, Web3Network::Ethereum) |
			// Gtc
			(Self::Gtc, Web3Network::Ethereum) |
			// Nfp
			(Self::Nfp, Web3Network::Bsc) |
			// Sol
			(Self::Sol, Web3Network::Bsc) | (Self::Sol, Web3Network::Ethereum) => 18,
			// Ton
			(Self::Ton, Web3Network::Bsc) | (Self::Ton, Web3Network::Ethereum) |
			// Mcrt
			(Self::Mcrt, Web3Network::Bsc) | (Self::Mcrt, Web3Network::Ethereum) => 9,
			// Wbtc
			(Self::Wbtc, Web3Network::Bsc) | (Self::Wbtc, Web3Network::Ethereum) |
			// Mcrt
			(Self::Mcrt, Web3Network::Solana) |
			// Btc
			(Self::Btc, Web3Network::BitcoinP2tr) | (Self::Btc, Web3Network::BitcoinP2pkh) |
			 (Self::Btc, Web3Network::BitcoinP2sh) | (Self::Btc, Web3Network::BitcoinP2wpkh) |
			  (Self::Btc, Web3Network::BitcoinP2wsh) => 8,
			// Usdc
			(Self::Usdc, Web3Network::Ethereum) |
			// Usdt
			(Self::Usdt, Web3Network::Ethereum) |
			// Trx
			(Self::Trx, Web3Network::Bsc) | (Self::Trx, Web3Network::Ethereum) => 6,
			// Gusd
			(Self::Gusd, Web3Network::Ethereum) => 2,
			_ => 1,
		};

		10_u64.pow(decimals)
	}
}

pub trait TokenHoldingAmountRange {
	fn get_token_holding_amount_range(&self) -> Vec<f64>;
}

impl TokenHoldingAmountRange for Web3TokenType {
	fn get_token_holding_amount_range(&self) -> Vec<f64> {
		match self {
			Self::Mcrt => vec![0.0, 2000.0, 10000.0, 50000.0, 150000.0, 500000.0],
			_ => vec![0.0, 1.0, 50.0, 100.0, 200.0, 500.0, 800.0, 1200.0, 1600.0, 3000.0],
		}
	}
}
