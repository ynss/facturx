//! Factur-X profile detection from the invoice's guideline identifier.

use crate::xml::{RAM, RSM, first_text};

const CII_ROOT: (&str, &str) = (RSM, "CrossIndustryInvoice");

const GUIDELINE_PATH: &[(&str, &str)] = &[
    (RSM, "ExchangedDocumentContext"),
    (RAM, "GuidelineSpecifiedDocumentContextParameter"),
    (RAM, "ID"),
];

// `_FX`: Factur-X spelling, `_ZF`: ZUGFeRD spelling of the same identifier.
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
    /// The guideline identifier is missing, empty, or not a known profile.
    Unknown,
}

impl Profile {
    /// The profile a CII invoice declares in its guideline identifier.
    ///
    /// `None` when the text is not a readable CII invoice. A missing, empty or unknown
    /// identifier gives `Some(Profile::Unknown)`; the identifier is never trimmed.
    #[must_use]
    pub fn from_xml(xml: &str) -> Option<Profile> {
        let guideline = first_text(xml, CII_ROOT, GUIDELINE_PATH)?;
        Some(Profile::from_guideline(&guideline))
    }

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
    pub fn name(&self) -> &'static str {
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

#[cfg(test)]
mod tests {
    use super::*;

    // Literal strings, not the constants, so a typo in a constant fails the test.
    #[test]
    fn detects_known_profiles_in_both_spellings() {
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
        let padded = "\n      urn:cen.eu:en16931:2017\n    ";
        assert_eq!(Profile::from_xml(&invoice(padded)), Some(Profile::Unknown));
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
    fn from_xml_reads_the_whole_text_around_comments() {
        assert_eq!(
            Profile::from_xml(&invoice("<!--c-->urn:cen.eu:en16931:2017")),
            Some(Profile::En16931)
        );
        assert_eq!(
            Profile::from_xml(&invoice("urn:cen.eu:<!--c-->en16931:2017")),
            Some(Profile::En16931)
        );
        assert_eq!(
            Profile::from_xml(&invoice("<![CDATA[urn:cen.eu:en16931:2017]]>")),
            Some(Profile::En16931)
        );
    }

    #[test]
    fn from_xml_is_none_when_not_a_readable_cii_invoice() {
        assert_eq!(Profile::from_xml("not xml at all"), None);
        assert_eq!(
            Profile::from_xml(
                r#"<Invoice xmlns="urn:oasis:names:specification:ubl:schema:xsd:Invoice-2"/>"#
            ),
            None
        );
        let id = "urn:cen.eu:en16931:2017";
        let no_namespace = format!(
            "<Pizza><ExchangedDocumentContext><GuidelineSpecifiedDocumentContextParameter><ID>{id}</ID></GuidelineSpecifiedDocumentContextParameter></ExchangedDocumentContext></Pizza>"
        );
        assert_eq!(Profile::from_xml(&no_namespace), None);
        let wrong_namespace = format!(
            r#"<x:CrossIndustryInvoice xmlns:x="urn:evil"><x:ExchangedDocumentContext><x:GuidelineSpecifiedDocumentContextParameter><x:ID>{id}</x:ID></x:GuidelineSpecifiedDocumentContextParameter></x:ExchangedDocumentContext></x:CrossIndustryInvoice>"#
        );
        assert_eq!(Profile::from_xml(&wrong_namespace), None);
        let other_root = invoice(id).replace("rsm:CrossIndustryInvoice", "rsm:Other");
        assert_eq!(Profile::from_xml(&other_root), None);
        let with_doctype = format!("<!DOCTYPE x>{}", invoice(id));
        assert_eq!(Profile::from_xml(&with_doctype), None);
    }

    #[test]
    fn from_xml_is_unknown_without_a_guideline() {
        let no_guideline = r#"<rsm:CrossIndustryInvoice
    xmlns:rsm="urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100">
  <rsm:ExchangedDocumentContext/>
</rsm:CrossIndustryInvoice>"#;
        assert_eq!(Profile::from_xml(no_guideline), Some(Profile::Unknown));
        assert_eq!(Profile::from_xml(&invoice("")), Some(Profile::Unknown));
        let id_in_rsm = invoice("urn:cen.eu:en16931:2017").replace("ram:ID", "rsm:ID");
        assert_eq!(Profile::from_xml(&id_in_rsm), Some(Profile::Unknown));
    }
}
