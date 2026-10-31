//! Factur-X for the French e-invoicing reform.
//!
//! Detect which Factur-X profile an invoice declares in its guideline identifier
//! (business term BT-24 of EN 16931), from the CII XML embedded in a Factur-X
//! PDF, with [`Profile::from_xml`]. Identifiers in their `ZUGFeRD` spelling (the
//! German name of the same standard) are recognised too.
//!
//! ```
//! use facturx::Profile;
//!
//! let xml = r#"<rsm:CrossIndustryInvoice
//!     xmlns:rsm="urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100"
//!     xmlns:ram="urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100">
//!   <rsm:ExchangedDocumentContext>
//!     <ram:GuidelineSpecifiedDocumentContextParameter>
//!       <ram:ID>urn:cen.eu:en16931:2017</ram:ID>
//!     </ram:GuidelineSpecifiedDocumentContextParameter>
//!   </rsm:ExchangedDocumentContext>
//! </rsm:CrossIndustryInvoice>"#;
//!
//! assert_eq!(Profile::from_xml(xml), Some(Profile::En16931));
//! ```

mod profile;
mod xml;

pub use profile::Profile;
