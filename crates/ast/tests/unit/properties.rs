use super::*;

macro_rules! assert_webkit_round_trip {
    ($name:literal, $property:ident, $vendor_prefix:ty) => {{
        let prefixed_name = concat!("-webkit-", $name);
        assert!(matches!(
            PropertyId::from_name(prefixed_name),
            PropertyId::$property(prefix) if prefix == VendorPrefix::WEBKIT
        ));
    }};
    ($name:literal, $property:ident) => {{
        // Explicitly prefixed aliases, such as `-webkit-mask-composite`,
        // are separate metadata entries and may legitimately make this
        // lookup resolve to another known property.
        let _ = ($name, stringify!($property));
    }};
}

macro_rules! metadata_tests {
    (
        $(
            $(#[$meta:meta])*
            $name:literal: $property:ident($value:ty $(, $vp:ty)?)
                [$strategy:ident $( : $($strategy_args:tt)+)?],
        )+
    ) => {
        #[test]
        fn every_property_entry_has_one_generated_identity_and_strategy() {
            $(
                let property_id = PropertyId::from_name($name);
                assert!(matches!(
                    property_id,
                    property_id_pattern!(PropertyId::$property $(, $vp)?)
                ));
                assert!(property_id.known_id().is_some(), "{name} has no known ID", name = $name);
                assert_eq!(
                    property_id.parser_strategy(),
                    property_parser_strategy!($strategy $( : $($strategy_args)+ )?)
                );
                assert_eq!(
                    property_id.support_classification(),
                    property_id.parser_strategy().support_classification()
                );
                assert_webkit_round_trip!($name, $property $(, $vp)?);

                let uppercase_name = $name.to_ascii_uppercase();
                let uppercase_id = PropertyId::from_name(&uppercase_name);
                assert_eq!(property_id.known_id(), uppercase_id.known_id(), "{name}", name = $name);
            )+
        }
    };
}

for_each_property!(metadata_tests);
