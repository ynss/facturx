# Changelog

All notable changes to this project will be documented in this file.

## [0.1.1] - 2026-11-01

### Fixed

- `Profile::from_xml` checks XML namespaces and the root element.

### Changed

- `Profile::from_xml` returns `None` only when the input is not a readable CII invoice; a
  missing or empty identifier gives `Profile::Unknown`.

## [0.1.0] - 2026-10-31

Initial release.

### Added

- Detect a Factur-X or ZUGFeRD profile from CII XML business term BT-24 with `Profile::from_xml`.
- Recognize the MINIMUM, BASIC WL, BASIC, EN 16931, EXTENDED, and French EXTENDED-CTC-FR profiles.
- Recognize both Factur-X and ZUGFeRD guideline identifier spellings.
- Provide profile display names through `Profile::name`.

### Scope

- PDF extraction is not included; callers provide the embedded XML.
- XML input is parsed to locate BT-24 but is not fully schema- or Schematron-validated.
