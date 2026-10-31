//! Factur-X profile detection from the invoice's guideline identifier.

use crate::xml::first_text_by_local;

/// A Factur-X profile, as named by the invoice's guideline identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Profile {
    Minimum,
    BasicWl,
    Basic,
    En16931,
    Extended,
    /// The French CTC profile.
    ExtendedCtcFr,
    /// The guideline identifier is present but is not a Factur-X profile.
    Unknown,
}

impl Profile {
    /// Read the profile an invoice declares in its CII XML (the XML inside a
    /// Factur-X PDF), from its guideline identifier.
    ///
    /// Returns `None` when the text is not well-formed XML or declares no
    /// guideline identifier; otherwise the profile, which is
    /// `Some(Profile::Unknown)` when the identifier is not a known profile. The
    /// identifier is read as is, never trimmed.
    #[must_use]
    pub fn from_xml(xml: &str) -> Option<Profile> {
        let guideline = first_text_by_local(xml, GUIDELINE_PATH)?;
        Some(Profile::from_guideline(&guideline))
    }

    /// Work out the profile from the guideline identifier. Every valid Factur-X
    /// profile is recognised; anything else is `Unknown`. The match is exact, as
    /// in the official rules: a padded value is `Unknown`.
    fn from_guideline(guideline: &str) -> Profile {
        match guideline {
            MINIMUM_FX | MINIMUM_ZF => Profile::Minimum,
            BASICWL_FX | BASICWL_ZF => Profile::BasicWl,
            BASIC_FX | BASIC_ZF => Profile::Basic,
            EN16931 => Profile::En16931,
            EXTENDED_FX | EXTENDED_ZF => Profile::Extended,
            EXTENDED_CTC_FR => Profile::ExtendedCtcFr,
            _ => Profile::Unknown,
        }
    }

    /// A short display name, e.g. `"EN16931"`.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Profile::Minimum => "MINIMUM",
            Profile::BasicWl => "BASIC WL",
            Profile::Basic => "BASIC",
            Profile::En16931 => "EN16931",
            Profile::Extended => "EXTENDED",
            Profile::ExtendedCtcFr => "EXTENDED-CTC-FR",
            Profile::Unknown => "UNKNOWN",
        }
    }
}

const GUIDELINE_PATH: &[&str] = &[
    "ExchangedDocumentContext",
    "GuidelineSpecifiedDocumentContextParameter",
    "ID",
];

// Every guideline identifier, in both the Factur-X and the ZUGFeRD spelling (two
// names for the same standard). Verified against the spec and the example
// invoices.
const MINIMUM_FX: &str = "urn:factur-x.eu:1p0:minimum";
const MINIMUM_ZF: &str = "urn:zugferd.de:2p0:minimum";
const BASICWL_FX: &str = "urn:factur-x.eu:1p0:basicwl";
const BASICWL_ZF: &str = "urn:zugferd.de:2p0:basicwl";
const BASIC_FX: &str = "urn:cen.eu:en16931:2017#compliant#urn:factur-x.eu:1p0:basic";
const BASIC_ZF: &str = "urn:cen.eu:en16931:2017#compliant#urn:zugferd.de:2p0:basic";
const EN16931: &str = "urn:cen.eu:en16931:2017";
const EXTENDED_FX: &str = "urn:cen.eu:en16931:2017#conformant#urn:factur-x.eu:1p0:extended";
const EXTENDED_ZF: &str = "urn:cen.eu:en16931:2017#conformant#urn:zugferd.de:2p0:extended";
const EXTENDED_CTC_FR: &str =
    "urn:cen.eu:en16931:2017#conformant#urn.cpro.gouv.fr:1p0:extended-ctc-fr";

#[cfg(test)]
mod tests {
    use super::*;

    // Literal identifiers copied from the official spec files, not the constants
    // above, so a typo in a constant makes this test fail.
    #[test]
    fn detects_every_valid_profile_in_both_spellings() {
        assert_eq!(
            Profile::from_guideline("urn:factur-x.eu:1p0:minimum"),
            Profile::Minimum
        );
        assert_eq!(
            Profile::from_guideline("urn:zugferd.de:2p0:minimum"),
            Profile::Minimum
        );
        assert_eq!(
            Profile::from_guideline("urn:factur-x.eu:1p0:basicwl"),
            Profile::BasicWl
        );
        assert_eq!(
            Profile::from_guideline("urn:zugferd.de:2p0:basicwl"),
            Profile::BasicWl
        );
        assert_eq!(
            Profile::from_guideline("urn:cen.eu:en16931:2017#compliant#urn:factur-x.eu:1p0:basic"),
            Profile::Basic
        );
        assert_eq!(
            Profile::from_guideline("urn:cen.eu:en16931:2017#compliant#urn:zugferd.de:2p0:basic"),
            Profile::Basic
        );
        assert_eq!(
            Profile::from_guideline("urn:cen.eu:en16931:2017"),
            Profile::En16931
        );
        assert_eq!(
            Profile::from_guideline(
                "urn:cen.eu:en16931:2017#conformant#urn:factur-x.eu:1p0:extended"
            ),
            Profile::Extended
        );
        assert_eq!(
            Profile::from_guideline(
                "urn:cen.eu:en16931:2017#conformant#urn:zugferd.de:2p0:extended"
            ),
            Profile::Extended
        );
        assert_eq!(
            Profile::from_guideline(
                "urn:cen.eu:en16931:2017#conformant#urn.cpro.gouv.fr:1p0:extended-ctc-fr"
            ),
            Profile::ExtendedCtcFr
        );
    }

    #[test]
    fn unrecognised_guideline_is_unknown() {
        assert_eq!(
            Profile::from_guideline("urn:example:not-a-profile"),
            Profile::Unknown
        );
        assert_eq!(Profile::from_guideline(""), Profile::Unknown);
    }

    /// A minimal CII invoice whose guideline identifier is `guideline`.
    fn invoice(guideline: &str) -> String {
        format!(
            r#"<rsm:CrossIndustryInvoice
    xmlns:rsm="urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100"
    xmlns:ram="urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100">
  <rsm:ExchangedDocumentContext>
    <ram:GuidelineSpecifiedDocumentContextParameter>
      <ram:ID>{guideline}</ram:ID>
    </ram:GuidelineSpecifiedDocumentContextParameter>
  </rsm:ExchangedDocumentContext>
</rsm:CrossIndustryInvoice>"#
        )
    }

    #[test]
    fn from_xml_reads_the_guideline() {
        assert_eq!(
            Profile::from_xml(&invoice("urn:cen.eu:en16931:2017")),
            Some(Profile::En16931)
        );
        assert_eq!(
            Profile::from_xml(&invoice("urn:factur-x.eu:1p0:minimum")),
            Some(Profile::Minimum)
        );
    }

    #[test]
    fn from_xml_does_not_trim_the_guideline() {
        // The official rule reads the identifier as is and rejects a padded value
        // (FX-SCH-A-000556), so a padded value is not a profile.
        let padded = "\n      urn:cen.eu:en16931:2017\n    ";
        assert_eq!(Profile::from_xml(&invoice(padded)), Some(Profile::Unknown));
        // Blank is a wrong value too (FX-SCH-A-000556), not a missing one.
        assert_eq!(Profile::from_xml(&invoice("   ")), Some(Profile::Unknown));
    }

    #[test]
    fn from_xml_ignores_prefixes() {
        let xml = r#"<a:CrossIndustryInvoice
    xmlns:a="urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100"
    xmlns:b="urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100">
  <a:ExchangedDocumentContext>
    <b:GuidelineSpecifiedDocumentContextParameter>
      <b:ID>urn:cen.eu:en16931:2017</b:ID>
    </b:GuidelineSpecifiedDocumentContextParameter>
  </a:ExchangedDocumentContext>
</a:CrossIndustryInvoice>"#;
        assert_eq!(Profile::from_xml(xml), Some(Profile::En16931));
    }

    #[test]
    fn from_xml_is_none_without_a_guideline() {
        let no_guideline = r#"<rsm:CrossIndustryInvoice
    xmlns:rsm="urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100">
  <rsm:ExchangedDocumentContext/>
</rsm:CrossIndustryInvoice>"#;
        assert_eq!(Profile::from_xml(no_guideline), None);
        // An empty identifier element is missing (BR-01).
        assert_eq!(Profile::from_xml(&invoice("")), None);
        assert_eq!(Profile::from_xml("not xml at all"), None);
        assert_eq!(
            Profile::from_xml(
                r#"<Invoice xmlns="urn:oasis:names:specification:ubl:schema:xsd:Invoice-2"/>"#
            ),
            None
        );
    }
}
