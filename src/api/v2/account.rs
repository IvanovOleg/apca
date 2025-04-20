// Copyright (C) 2019-2024 The apca Developers
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Deref;

use chrono::DateTime;
use chrono::Utc;

use num_decimal::Num;

use serde::Deserialize;
use serde::Serialize;

use uuid::Uuid;

use crate::Str;

/// A type representing an account ID.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Id(pub Uuid);

impl Deref for Id {
  type Target = Uuid;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

/// An enumeration of the various states an account can be in.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub enum Status {
  /// The account is onboarding.
  #[serde(rename = "ONBOARDING")]
  Onboarding,
  /// The account application submission failed for some reason.
  #[serde(rename = "SUBMISSION_FAILED")]
  SubmissionFailed,
  /// The account application has been submitted for review.
  #[serde(rename = "SUBMITTED")]
  Submitted,
  /// The account information is being updated.
  #[serde(rename = "ACCOUNT_UPDATED")]
  Updating,
  /// The final account approval is pending.
  #[serde(rename = "APPROVAL_PENDING")]
  ApprovalPending,
  /// The account is active for trading.
  #[serde(rename = "ACTIVE")]
  Active,
  /// The account application has been rejected.
  #[serde(rename = "REJECTED")]
  Rejected,
  /// Any other account status that we have not accounted for.
  ///
  /// Note that having any such status should be considered a bug.
  #[doc(hidden)]
  #[serde(other, rename(serialize = "unknown"))]
  Unknown,
}

/// An object as returned by the /v2/account endpoint.
// TODO: The `sma` field is not yet hooked up.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Account {
  /// Account ID.
  #[serde(rename = "id")]
  pub id: Id,
  /// Admin configurations for the account.
  #[serde(rename = "admin_configurations")]
  pub admin_configurations: serde_json::Value,
  /// User configurations for the account.
  #[serde(rename = "user_configurations")]
  pub user_configurations: Option<serde_json::Value>,
  /// Account number.
  #[serde(rename = "account_number")]
  pub account_number: String,
  /// The account's status.
  #[serde(rename = "status")]
  pub status: Status,
  /// The account's crypto status.
  #[serde(rename = "crypto_status")]
  pub crypto_status: String,
  /// The currency the account uses.
  #[serde(rename = "currency")]
  pub currency: String,
  /// Cash balance.
  #[serde(rename = "cash")]
  pub cash: Num,
  /// Portfolio value (equity + cash)
  #[serde(rename = "portfolio_value")]
  pub portfolio_value: Num,
  /// Whether or not the account has been flagged as a pattern day
  /// trader.
  #[serde(rename = "pattern_day_trader")]
  pub day_trader: bool,
  /// Whether or not the user has suspended trading operations.
  #[serde(rename = "trade_suspended_by_user")]
  pub trading_suspended: bool,
  /// If true, the account is not allowed to place orders.
  #[serde(rename = "trading_blocked")]
  pub trading_blocked: bool,
  /// If true, the account is not allowed to request money transfers.
  #[serde(rename = "transfers_blocked")]
  pub transfers_blocked: bool,
  /// If true, the account activity by user is prohibited.
  #[serde(rename = "account_blocked")]
  pub account_blocked: bool,
  /// Timestamp this account was created at.
  #[serde(rename = "created_at")]
  pub created_at: DateTime<Utc>,
  /// Flag to denote whether or not the account is permitted to short.
  #[serde(rename = "shorting_enabled")]
  pub shorting_enabled: bool,
  /// Real-time mark-to-market value of all long positions held in the
  /// account.
  #[serde(rename = "long_market_value")]
  pub market_value_long: Num,
  /// Real-time mark-to-market value of all short positions held in the
  /// account.
  #[serde(rename = "short_market_value")]
  pub market_value_short: Num,
  /// The sum of `cash`, `market_value_long`, and `market_value_short`.
  #[serde(rename = "equity")]
  pub equity: Num,
  /// Equity as of previous trading day at 16:00:00 ET.
  #[serde(rename = "last_equity")]
  pub last_equity: Num,
  /// Buying power multiplier that represents account margin
  /// classification. Valid values are:
  /// - 1: the standard limited margin account with 1x buying power
  /// - 2: regular margin account with 2x intra day and overnight buying
  ///      power (the default for all non-pattern-day-trader accounts
  ///      with USD 2000 or more equity),
  /// - 4: pattern day trader account with 4x intra day buying power and
  ///      2x regular overnight buying power
  #[serde(rename = "multiplier")]
  pub multiplier: Num,
  /// The currently available buying power. Calculated based on the
  /// multiplier:
  /// - 1: cash
  /// - 2: max(equity – initial_margin, 0) * 2
  /// - 4: (last_equity - (last) maintenance_margin) * 4
  #[serde(rename = "buying_power")]
  pub buying_power: Num,
  /// Regulatory buying power.
  #[serde(rename = "regt_buying_power")]
  pub regt_buying_power: Num,
  /// Day trading buying power.
  #[serde(rename = "daytrading_buying_power")]
  pub daytrading_buying_power: Num,
  /// Options buying power.
  #[serde(rename = "options_buying_power")]
  pub options_buying_power: Num,
  /// Effective buying power.
  #[serde(rename = "effective_buying_power")]
  pub effective_buying_power: Num,
  /// Non-marginable buying power.
  #[serde(rename = "non_marginable_buying_power")]
  pub non_marginable_buying_power: Num,
  /// Beginning of day day trading buying power.
  #[serde(rename = "bod_dtbp")]
  pub bod_dtbp: Num,
  /// Accrued fees.
  #[serde(rename = "accrued_fees")]
  pub accrued_fees: Num,
  /// Pending transfer in.
  #[serde(rename = "pending_transfer_in", default)]
  pub pending_transfer_in: Num,
  /// Position market value.
  #[serde(rename = "position_market_value")]
  pub position_market_value: Num,
  /// Initial margin requirement (this value is continuously updated).
  #[serde(rename = "initial_margin")]
  pub initial_margin: Num,
  /// Maintenance margin requirement (this value is continuously updated).
  #[serde(rename = "maintenance_margin")]
  pub maintenance_margin: Num,
  /// Last maintenance margin.
  #[serde(rename = "last_maintenance_margin")]
  pub last_maintenance_margin: Num,
  /// Special Memorandum Account (SMA) balance
  #[serde(rename = "sma")]
  pub sma: Num,
  /// The current number of day trades that have been made in the last
  /// five trading days (including today).
  #[serde(rename = "daytrade_count")]
  pub daytrade_count: u64,
  /// Balance as of date.
  #[serde(rename = "balance_asof")]
  pub balance_asof: String,
  /// Crypto tier.
  #[serde(rename = "crypto_tier")]
  pub crypto_tier: u64,
  /// Options trading level.
  #[serde(rename = "options_trading_level")]
  pub options_trading_level: u64,
  /// Intraday adjustments.
  #[serde(rename = "intraday_adjustments")]
  pub intraday_adjustments: Num,
  /// Pending regulatory TAF fees.
  #[serde(rename = "pending_reg_taf_fees")]
  pub pending_reg_taf_fees: Num,
  /// The type is non-exhaustive and open to extension.
  #[doc(hidden)]
  #[serde(skip)]
  pub _non_exhaustive: (),
}

Endpoint! {
  /// The representation of a GET request to the /v2/account endpoint.
  pub Get(()),
  Ok => Account, [
    /// The account information was retrieved successfully.
    /* 200 */ OK,
  ],
  Err => GetError, []

  #[inline]
  fn path(_input: &Self::Input) -> Str {
    "/v2/account".into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use serde_json::from_str as from_json;
  use serde_json::to_string as to_json;

  use test_log::test;

  use uuid::Uuid;

  use crate::api::API_BASE_URL;
  use crate::api_info::ApiInfo;
  use crate::Client;
  use crate::RequestError;

  /// Make sure that we can deserialize and serialize the reference
  /// account object.
  #[test]
  fn deserialize_serialize_reference_account() {
    let json = r#"{
  "id": "904837e3-3b76-47ec-b432-046db621571b",
  "admin_configurations": {},
  "user_configurations": null,
  "account_number": "PALPACA_123",
  "status": "ACTIVE",
  "crypto_status": "ACTIVE",
  "currency": "USD",
  "buying_power": "0.0",
  "regt_buying_power": "0.0",
  "daytrading_buying_power": "0.0",
  "options_buying_power": "0.0",
  "effective_buying_power": "0.0",
  "non_marginable_buying_power": "0.0",
  "bod_dtbp": "0.0",
  "cash": "1000.00",
  "accrued_fees": "0.0",
  "pending_transfer_in": "0.0",
  "portfolio_value": "5000.00",
  "pattern_day_trader": false,
  "trade_suspended_by_user": false,
  "trading_blocked": false,
  "transfers_blocked": false,
  "account_blocked": false,
  "created_at": "2018-10-01T13:35:25Z",
  "shorting_enabled": true,
  "multiplier": "2",
  "long_market_value": "7000.00",
  "short_market_value": "-3000.00",
  "position_market_value": "4000.00",
  "equity": "5000.00",
  "last_equity": "5000.00",
  "initial_margin": "5000.00",
  "maintenance_margin": "3000.00",
  "last_maintenance_margin": "3000.00",
  "sma": "0.0",
  "daytrade_count": 0,
  "balance_asof": "2018-10-01",
  "crypto_tier": 1,
  "options_trading_level": 2,
  "intraday_adjustments": "0.0",
  "pending_reg_taf_fees": "0.0"
}"#;

    let acc =
      from_json::<Account>(&to_json(&from_json::<Account>(json).unwrap()).unwrap()).unwrap();

    let id = Id(Uuid::parse_str("904837e3-3b76-47ec-b432-046db621571b").unwrap());
    assert_eq!(acc.id, id);
    assert_eq!(acc.status, Status::Active);
    assert_eq!(acc.currency, "USD");
    assert_eq!(acc.buying_power, Num::from(0));
    assert!(!acc.trading_blocked);
    assert_eq!(
      acc.created_at,
      DateTime::parse_from_rfc3339("2018-10-01T13:35:25Z").unwrap()
    );
    assert_eq!(acc.market_value_long, Num::from(7000));
    assert_eq!(acc.market_value_short, Num::from(-3000));
    assert_eq!(acc.equity, Num::from(5000));
    assert_eq!(acc.last_equity, Num::from(5000));
    assert_eq!(acc.maintenance_margin, Num::from(3000));
    assert_eq!(acc.daytrade_count, 0);

    // Test new fields
    assert_eq!(acc.account_number, "PALPACA_123");
    assert_eq!(acc.crypto_status, "ACTIVE");
    assert_eq!(acc.regt_buying_power, Num::from(0));
    assert_eq!(acc.daytrading_buying_power, Num::from(0));
    assert_eq!(acc.options_buying_power, Num::from(0));
    assert_eq!(acc.effective_buying_power, Num::from(0));
    assert_eq!(acc.non_marginable_buying_power, Num::from(0));
    assert_eq!(acc.bod_dtbp, Num::from(0));
    assert_eq!(acc.accrued_fees, Num::from(0));
    assert_eq!(acc.pending_transfer_in, Num::from(0));
    assert_eq!(acc.position_market_value, Num::from(4000));
    assert_eq!(acc.last_maintenance_margin, Num::from(3000));
    assert_eq!(acc.balance_asof, "2018-10-01");
    assert_eq!(acc.crypto_tier, 1);
    assert_eq!(acc.options_trading_level, 2);
    assert_eq!(acc.intraday_adjustments, Num::from(0));
    assert_eq!(acc.pending_reg_taf_fees, Num::from(0));
  }

  /// Test that we can retrieve information about the account.
  #[test(tokio::test)]
  async fn request_account() {
    let api_info = ApiInfo::from_env().unwrap();
    let client = Client::new(api_info);
    let account = client.issue::<Get>(&()).await.unwrap();

    assert_eq!(account.currency, "USD");
    assert!(!account.account_blocked);

    let multiplier = account.multiplier.to_u64().unwrap();
    assert!(
      multiplier == 1 || multiplier == 2 || multiplier == 4,
      "{}",
      multiplier,
    );
  }

  /// Check that we get back the expected error when requesting account
  /// data with invalid credentials.
  #[test(tokio::test)]
  async fn request_account_with_invalid_credentials() {
    let api_info = ApiInfo::from_parts(API_BASE_URL, "invalid", "invalid-too").unwrap();
    let client = Client::new(api_info);
    let result = client.issue::<Get>(&()).await;

    let err = result.unwrap_err();
    match err {
      RequestError::Endpoint(GetError::NotPermitted(_)) => (),
      e => panic!("received unexpected error: {e:?}"),
    }
  }
}
