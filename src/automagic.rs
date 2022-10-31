use std::str::FromStr;
use xsd_macro_utils::{UtilsDefaultSerde, UtilsTupleIo, UtilsUnionSerDe};
use xsd_parser::generator::validator::Validate;
use xsd_types::types as xs;
use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct OpenDrive {
    #[yaserde(rename = "header")]
    pub header: open_drive::HeaderType,

    #[yaserde(rename = "road")]
    pub road: Vec<open_drive::RoadType>,

    #[yaserde(rename = "controller")]
    pub controller: Vec<open_drive::ControllerType>,

    #[yaserde(rename = "junction")]
    pub junction: Vec<open_drive::JunctionType>,

    #[yaserde(rename = "junctionGroup")]
    pub junction_group: Vec<open_drive::JunctionGroupType>,

    #[yaserde(rename = "station")]
    pub station: Vec<open_drive::StationType>,
}

impl Validate for OpenDrive {}

pub mod open_drive {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct HeaderType {
        #[yaserde(rename = "geoReference")]
        pub geo_reference: String,

        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "revMajor")]
        pub rev_major: Option<u16>,

        #[yaserde(attribute, rename = "revMinor")]
        pub rev_minor: Option<u16>,

        #[yaserde(attribute, rename = "name")]
        pub name: Option<String>,

        #[yaserde(attribute, rename = "version")]
        pub version: Option<f64>,

        #[yaserde(attribute, rename = "date")]
        pub date: Option<String>,

        #[yaserde(attribute, rename = "north")]
        pub north: Option<f64>,

        #[yaserde(attribute, rename = "south")]
        pub south: Option<f64>,

        #[yaserde(attribute, rename = "east")]
        pub east: Option<f64>,

        #[yaserde(attribute, rename = "west")]
        pub west: Option<f64>,

        #[yaserde(attribute, rename = "vendor")]
        pub vendor: Option<String>,
    }

    impl Validate for HeaderType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct RoadType {
        #[yaserde(rename = "link")]
        pub link: road_type::LinkType,

        #[yaserde(rename = "type")]
        pub _type: Vec<road_type::TypeType>,

        #[yaserde(rename = "planView")]
        pub plan_view: road_type::PlanViewType,

        #[yaserde(rename = "elevationProfile")]
        pub elevation_profile: road_type::ElevationProfileType,

        #[yaserde(rename = "lateralProfile")]
        pub lateral_profile: road_type::LateralProfileType,

        #[yaserde(rename = "lanes")]
        pub lanes: road_type::LanesType,

        #[yaserde(rename = "objects")]
        pub objects: road_type::ObjectsType,

        #[yaserde(rename = "signals")]
        pub signals: road_type::SignalsType,

        #[yaserde(rename = "surface")]
        pub surface: road_type::SurfaceType,

        #[yaserde(rename = "railroad")]
        pub railroad: road_type::RailroadType,

        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "name")]
        pub name: Option<String>,

        #[yaserde(attribute, rename = "length")]
        pub length: Option<f64>,

        #[yaserde(attribute, rename = "id")]
        pub id: Option<String>,

        #[yaserde(attribute, rename = "junction")]
        pub junction: Option<String>,
    }

    impl Validate for RoadType {}

    pub mod road_type {
        use super::*;

        #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct LinkType {
            #[yaserde(rename = "predecessor")]
            pub predecessor: link_type::PredecessorType,

            #[yaserde(rename = "successor")]
            pub successor: link_type::SuccessorType,

            #[yaserde(rename = "neighbor")]
            pub neighbor: Vec<link_type::NeighborType>,

            #[yaserde(rename = "userData")]
            pub user_data: Vec<UserData>,

            #[yaserde(rename = "include")]
            pub include: Vec<Include>,
        }

        impl Validate for LinkType {}

        pub mod link_type {
            use super::*;

            #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct PredecessorType {
                #[yaserde(attribute, rename = "elementType")]
                pub element_type: Option<ElementType>,

                #[yaserde(attribute, rename = "elementId")]
                pub element_id: Option<String>,

                #[yaserde(attribute, rename = "contactPoint")]
                pub contact_point: Option<ContactPoint>,
            }

            impl Validate for PredecessorType {}
            #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct SuccessorType {
                #[yaserde(attribute, rename = "elementType")]
                pub element_type: Option<ElementType>,

                #[yaserde(attribute, rename = "elementId")]
                pub element_id: Option<String>,

                #[yaserde(attribute, rename = "contactPoint")]
                pub contact_point: Option<ContactPoint>,
            }

            impl Validate for SuccessorType {}
            #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct NeighborType {
                #[yaserde(attribute, rename = "side")]
                pub side: Option<Side>,

                #[yaserde(attribute, rename = "elementId")]
                pub element_id: Option<String>,

                #[yaserde(attribute, rename = "direction")]
                pub direction: Option<Direction>,
            }

            impl Validate for NeighborType {}
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct TypeType {
            #[yaserde(rename = "speed")]
            pub speed: type_type::SpeedType,

            #[yaserde(rename = "userData")]
            pub user_data: Vec<UserData>,

            #[yaserde(rename = "include")]
            pub include: Vec<Include>,

            #[yaserde(attribute, rename = "s")]
            pub s: Option<f64>,

            #[yaserde(attribute, rename = "type")]
            pub _type: Option<RoadType>,
        }

        impl Validate for TypeType {}

        pub mod type_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct SpeedType {
                #[yaserde(attribute, rename = "max")]
                pub max: Option<Max>,

                #[yaserde(attribute, rename = "unit")]
                pub unit: Option<Unit>,
            }

            impl Validate for SpeedType {}
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct PlanViewType {
            #[yaserde(rename = "geometry")]
            pub geometry: Vec<plan_view_type::Geometry>,
        }

        impl Validate for PlanViewType {}

        pub mod plan_view_type {
            use super::*;

            #[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub enum GeometryChoice {
                #[yaserde(rename = "line")]
                // TODO: add stuff for line
                Line,
                #[yaserde(rename = "spiral")]
                // TODO: add stuff for spiral
                Spiral,
                #[yaserde(rename = "arc")]
                // TODO: add stuff for arc
                Arc,
                #[yaserde(rename = "poly3")]
                // TODO: add stuff for poly3
                Poly3,
                #[yaserde(rename = "paramPoly3")]
                // TODO: add stuff for parampoly3
                ParamPoly3,
                #[yaserde(rename = "userData")]
                UserData(Vec<UserData>),
                #[yaserde(rename = "include")]
                Include(Vec<Include>),
                __Unknown__(String),
            }

            impl Default for GeometryChoice {
                fn default() -> GeometryChoice {
                    Self::__Unknown__("No valid variants".into())
                }
            }

            impl Validate for GeometryChoice {}
            #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
            pub struct ParamPoly3 {}

            impl Validate for ParamPoly3 {}

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct Geometry {
                #[yaserde(attribute, rename = "s")]
                pub s: Option<f64>,

                #[yaserde(attribute, rename = "x")]
                pub x: Option<f64>,

                #[yaserde(attribute, rename = "y")]
                pub y: Option<f64>,

                #[yaserde(attribute, rename = "hdg")]
                pub hdg: Option<f64>,

                #[yaserde(attribute, rename = "length")]
                pub length: Option<f64>,

                #[yaserde(child)]
                pub geometry_choice: GeometryChoice,
            }

            impl Validate for Geometry {}
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct ElevationProfileType {
            #[yaserde(rename = "elevation")]
            pub elevation: Vec<elevation_profile_type::ElevationType>,

            #[yaserde(rename = "userData")]
            pub user_data: Vec<UserData>,

            #[yaserde(rename = "include")]
            pub include: Vec<Include>,
        }

        impl Validate for ElevationProfileType {}

        pub mod elevation_profile_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ElevationType {
                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "s")]
                pub s: Option<f64>,

                #[yaserde(attribute, rename = "a")]
                pub a: Option<f64>,

                #[yaserde(attribute, rename = "b")]
                pub b: Option<f64>,

                #[yaserde(attribute, rename = "c")]
                pub c: Option<f64>,

                #[yaserde(attribute, rename = "d")]
                pub d: Option<f64>,
            }

            impl Validate for ElevationType {}
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct LateralProfileType {
            #[yaserde(rename = "superelevation")]
            pub superelevation: Vec<lateral_profile_type::SuperelevationType>,

            #[yaserde(rename = "crossfall")]
            pub crossfall: Vec<lateral_profile_type::CrossfallType>,

            #[yaserde(rename = "shape")]
            pub shape: Vec<lateral_profile_type::ShapeType>,

            #[yaserde(rename = "userData")]
            pub user_data: Vec<UserData>,

            #[yaserde(rename = "include")]
            pub include: Vec<Include>,
        }

        impl Validate for LateralProfileType {}

        pub mod lateral_profile_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct SuperelevationType {
                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "s")]
                pub s: Option<f64>,

                #[yaserde(attribute, rename = "a")]
                pub a: Option<f64>,

                #[yaserde(attribute, rename = "b")]
                pub b: Option<f64>,

                #[yaserde(attribute, rename = "c")]
                pub c: Option<f64>,

                #[yaserde(attribute, rename = "d")]
                pub d: Option<f64>,
            }

            impl Validate for SuperelevationType {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct CrossfallType {
                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "side")]
                pub side: Option<CrossfallSide>,

                #[yaserde(attribute, rename = "s")]
                pub s: Option<f64>,

                #[yaserde(attribute, rename = "a")]
                pub a: Option<f64>,

                #[yaserde(attribute, rename = "b")]
                pub b: Option<f64>,

                #[yaserde(attribute, rename = "c")]
                pub c: Option<f64>,

                #[yaserde(attribute, rename = "d")]
                pub d: Option<f64>,
            }

            impl Validate for CrossfallType {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ShapeType {
                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "s")]
                pub s: Option<f64>,

                #[yaserde(attribute, rename = "t")]
                pub t: Option<f64>,

                #[yaserde(attribute, rename = "a")]
                pub a: Option<f64>,

                #[yaserde(attribute, rename = "b")]
                pub b: Option<f64>,

                #[yaserde(attribute, rename = "c")]
                pub c: Option<f64>,

                #[yaserde(attribute, rename = "d")]
                pub d: Option<f64>,
            }

            impl Validate for ShapeType {}
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct LanesType {
            #[yaserde(rename = "laneOffset")]
            pub lane_offset: Vec<lanes_type::LaneOffsetType>,

            #[yaserde(rename = "laneSection")]
            pub lane_section: Vec<lanes_type::LaneSectionType>,
        }

        impl Validate for LanesType {}

        pub mod lanes_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct LaneOffsetType {
                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "s")]
                pub s: Option<f64>,

                #[yaserde(attribute, rename = "a")]
                pub a: Option<f64>,

                #[yaserde(attribute, rename = "b")]
                pub b: Option<f64>,

                #[yaserde(attribute, rename = "c")]
                pub c: Option<f64>,

                #[yaserde(attribute, rename = "d")]
                pub d: Option<f64>,
            }

            impl Validate for LaneOffsetType {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct LaneSectionType {
                #[yaserde(rename = "left")]
                pub left: lane_section_type::LeftType,

                #[yaserde(rename = "center")]
                pub center: lane_section_type::CenterType,

                #[yaserde(rename = "right")]
                pub right: lane_section_type::RightType,

                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "s")]
                pub s: Option<f64>,

                #[yaserde(attribute, rename = "singleSide")]
                pub single_side: Option<SingleSide>,
            }

            impl Validate for LaneSectionType {}

            pub mod lane_section_type {
                use super::*;

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct LeftType {
                    #[yaserde(rename = "lane")]
                    pub lane: Vec<Lane>,

                    #[yaserde(rename = "userData")]
                    pub user_data: Vec<UserData>,

                    #[yaserde(rename = "include")]
                    pub include: Vec<Include>,
                }

                impl Validate for LeftType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct CenterType {
                    #[yaserde(rename = "lane")]
                    pub lane: CenterLane,

                    #[yaserde(rename = "userData")]
                    pub user_data: Vec<UserData>,

                    #[yaserde(rename = "include")]
                    pub include: Vec<Include>,
                }

                impl Validate for CenterType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct RightType {
                    #[yaserde(rename = "lane")]
                    pub lane: Vec<Lane>,

                    #[yaserde(rename = "userData")]
                    pub user_data: Vec<UserData>,

                    #[yaserde(rename = "include")]
                    pub include: Vec<Include>,
                }

                impl Validate for RightType {}
            }
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct ObjectsType {
            #[yaserde(rename = "object")]
            pub object: Vec<objects_type::ObjectType>,

            #[yaserde(rename = "objectReference")]
            pub object_reference: Vec<objects_type::ObjectReferenceType>,

            #[yaserde(rename = "tunnel")]
            pub tunnel: Vec<objects_type::TunnelType>,

            #[yaserde(rename = "bridge")]
            pub bridge: Vec<objects_type::BridgeType>,

            #[yaserde(rename = "userData")]
            pub user_data: Vec<UserData>,

            #[yaserde(rename = "include")]
            pub include: Vec<Include>,
        }

        impl Validate for ObjectsType {}

        pub mod objects_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ObjectType {
                #[yaserde(rename = "repeat")]
                pub repeat: Vec<object_type::RepeatType>,

                #[yaserde(rename = "outline")]
                pub outline: object_type::OutlineType,

                #[yaserde(rename = "material")]
                pub material: object_type::MaterialType,

                #[yaserde(rename = "validity")]
                pub validity: Vec<LaneValidity>,

                #[yaserde(rename = "parkingSpace")]
                pub parking_space: ParkingSpace,

                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "type")]
                pub _type: Option<String>,

                #[yaserde(attribute, rename = "name")]
                pub name: Option<String>,

                #[yaserde(attribute, rename = "id")]
                pub id: Option<String>,

                #[yaserde(attribute, rename = "s")]
                pub s: Option<f64>,

                #[yaserde(attribute, rename = "t")]
                pub t: Option<f64>,

                #[yaserde(attribute, rename = "zOffset")]
                pub z_offset: Option<f64>,

                #[yaserde(attribute, rename = "validLength")]
                pub valid_length: Option<f64>,

                #[yaserde(attribute, rename = "orientation")]
                pub orientation: Option<Orientation>,

                #[yaserde(attribute, rename = "length")]
                pub length: Option<f64>,

                #[yaserde(attribute, rename = "width")]
                pub width: Option<f64>,

                #[yaserde(attribute, rename = "radius")]
                pub radius: Option<f64>,

                #[yaserde(attribute, rename = "height")]
                pub height: Option<f64>,

                #[yaserde(attribute, rename = "hdg")]
                pub hdg: Option<f64>,

                #[yaserde(attribute, rename = "pitch")]
                pub pitch: Option<f64>,

                #[yaserde(attribute, rename = "roll")]
                pub roll: Option<f64>,
            }

            impl Validate for ObjectType {}

            pub mod object_type {
                use super::*;

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct RepeatType {
                    #[yaserde(rename = "userData")]
                    pub user_data: Vec<UserData>,

                    #[yaserde(rename = "include")]
                    pub include: Vec<Include>,

                    #[yaserde(attribute, rename = "s")]
                    pub s: Option<f64>,

                    #[yaserde(attribute, rename = "length")]
                    pub length: Option<f64>,

                    #[yaserde(attribute, rename = "distance")]
                    pub distance: Option<f64>,

                    #[yaserde(attribute, rename = "tStart")]
                    pub t_start: Option<f64>,

                    #[yaserde(attribute, rename = "tEnd")]
                    pub t_end: Option<f64>,

                    #[yaserde(attribute, rename = "widthStart")]
                    pub width_start: Option<f64>,

                    #[yaserde(attribute, rename = "widthEnd")]
                    pub width_end: Option<f64>,

                    #[yaserde(attribute, rename = "heightStart")]
                    pub height_start: Option<f64>,

                    #[yaserde(attribute, rename = "heightEnd")]
                    pub height_end: Option<f64>,

                    #[yaserde(attribute, rename = "zOffsetStart")]
                    pub z_offset_start: Option<f64>,

                    #[yaserde(attribute, rename = "zOffsetEnd")]
                    pub z_offset_end: Option<f64>,
                }

                impl Validate for RepeatType {}
                #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct OutlineType {
                    #[yaserde(rename = "outlineChoice")]
                    pub outline_choice: outline_type::OutlineChoice,
                }

                impl Validate for OutlineType {}

                pub mod outline_type {
                    use super::*;

                    #[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
                    #[yaserde()]

                    pub enum OutlineChoice {
                        #[yaserde(rename = "cornerRoad")]
                        CornerRoad,
                        #[yaserde(rename = "cornerLocal")]
                        CornerLocal,
                        #[yaserde(rename = "userData")]
                        UserData(Vec<UserData>),
                        #[yaserde(rename = "include")]
                        Include(Vec<Include>),
                        __Unknown__(String),
                    }

                    impl Default for OutlineChoice {
                        fn default() -> OutlineChoice {
                            Self::__Unknown__("No valid variants".into())
                        }
                    }

                    impl Validate for OutlineChoice {}
                }

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct MaterialType {
                    #[yaserde(rename = "userData")]
                    pub user_data: Vec<UserData>,

                    #[yaserde(rename = "include")]
                    pub include: Vec<Include>,

                    #[yaserde(attribute, rename = "surface")]
                    pub surface: Option<String>,

                    #[yaserde(attribute, rename = "friction")]
                    pub friction: Option<f64>,

                    #[yaserde(attribute, rename = "roughness")]
                    pub roughness: Option<f64>,
                }

                impl Validate for MaterialType {}
            }

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct ObjectReferenceType {
                #[yaserde(rename = "validity")]
                pub validity: Vec<LaneValidity>,

                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "s")]
                pub s: Option<f64>,

                #[yaserde(attribute, rename = "t")]
                pub t: Option<f64>,

                #[yaserde(attribute, rename = "id")]
                pub id: Option<String>,

                #[yaserde(attribute, rename = "zOffset")]
                pub z_offset: Option<f64>,

                #[yaserde(attribute, rename = "validLength")]
                pub valid_length: Option<f64>,

                #[yaserde(attribute, rename = "orientation")]
                pub orientation: Option<Orientation>,
            }

            impl Validate for ObjectReferenceType {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct TunnelType {
                #[yaserde(rename = "validity")]
                pub validity: Vec<LaneValidity>,

                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "s")]
                pub s: Option<f64>,

                #[yaserde(attribute, rename = "length")]
                pub length: Option<f64>,

                #[yaserde(attribute, rename = "name")]
                pub name: Option<String>,

                #[yaserde(attribute, rename = "id")]
                pub id: Option<String>,

                #[yaserde(attribute, rename = "type")]
                pub _type: Option<TunnelEnum>,

                #[yaserde(attribute, rename = "lighting")]
                pub lighting: Option<f64>,

                #[yaserde(attribute, rename = "daylight")]
                pub daylight: Option<f64>,
            }

            impl Validate for TunnelType {}
            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct BridgeType {
                #[yaserde(rename = "validity")]
                pub validity: Vec<LaneValidity>,

                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "s")]
                pub s: Option<f64>,

                #[yaserde(attribute, rename = "length")]
                pub length: Option<f64>,

                #[yaserde(attribute, rename = "name")]
                pub name: Option<String>,

                #[yaserde(attribute, rename = "id")]
                pub id: Option<String>,

                #[yaserde(attribute, rename = "type")]
                pub _type: Option<BridgeEnum>,
            }

            impl Validate for BridgeType {}
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct SignalsType {
            #[yaserde(rename = "signal")]
            pub signal: Vec<signals_type::SignalType>,

            #[yaserde(rename = "signalReference")]
            pub signal_reference: Vec<signals_type::SignalReferenceType>,

            #[yaserde(rename = "userData")]
            pub user_data: Vec<UserData>,

            #[yaserde(rename = "include")]
            pub include: Vec<Include>,
        }

        impl Validate for SignalsType {}

        pub mod signals_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct SignalType {
                #[yaserde(rename = "validity")]
                pub validity: Vec<LaneValidity>,

                #[yaserde(rename = "dependency")]
                pub dependency: Vec<signal_type::DependencyType>,

                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "s")]
                pub s: Option<f64>,

                #[yaserde(attribute, rename = "t")]
                pub t: Option<f64>,

                #[yaserde(attribute, rename = "id")]
                pub id: Option<String>,

                #[yaserde(attribute, rename = "name")]
                pub name: Option<String>,

                #[yaserde(attribute, rename = "dynamic")]
                pub dynamic: Option<Dynamic>,

                #[yaserde(attribute, rename = "orientation")]
                pub orientation: Option<Orientation>,

                #[yaserde(attribute, rename = "zOffset")]
                pub z_offset: Option<f64>,

                #[yaserde(attribute, rename = "country")]
                pub country: Option<String>,

                #[yaserde(attribute, rename = "type")]
                pub _type: Option<String>,

                #[yaserde(attribute, rename = "subtype")]
                pub subtype: Option<String>,

                #[yaserde(attribute, rename = "value")]
                pub value: Option<f64>,

                #[yaserde(attribute, rename = "unit")]
                pub unit: Option<Unit>,

                #[yaserde(attribute, rename = "height")]
                pub height: Option<f64>,

                #[yaserde(attribute, rename = "width")]
                pub width: Option<f64>,

                #[yaserde(attribute, rename = "text")]
                pub text: Option<String>,

                #[yaserde(attribute, rename = "hOffset")]
                pub h_offset: Option<f64>,

                #[yaserde(attribute, rename = "pitch")]
                pub pitch: Option<f64>,

                #[yaserde(attribute, rename = "roll")]
                pub roll: Option<f64>,
            }

            impl Validate for SignalType {}

            pub mod signal_type {
                use super::*;

                #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct DependencyType {
                    #[yaserde(rename = "userData")]
                    pub user_data: Vec<UserData>,

                    #[yaserde(rename = "include")]
                    pub include: Vec<Include>,

                    #[yaserde(attribute, rename = "id")]
                    pub id: Option<String>,

                    #[yaserde(attribute, rename = "type")]
                    pub _type: Option<String>,
                }

                impl Validate for DependencyType {}
            }

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct SignalReferenceType {
                #[yaserde(rename = "validity")]
                pub validity: Vec<LaneValidity>,

                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "s")]
                pub s: Option<f64>,

                #[yaserde(attribute, rename = "t")]
                pub t: Option<f64>,

                #[yaserde(attribute, rename = "id")]
                pub id: Option<String>,

                #[yaserde(attribute, rename = "orientation")]
                pub orientation: Option<Orientation>,
            }

            impl Validate for SignalReferenceType {}
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct SurfaceType {
            #[yaserde(rename = "CRG")]
            pub crg: Vec<surface_type::Crgtype>,

            #[yaserde(rename = "userData")]
            pub user_data: Vec<UserData>,

            #[yaserde(rename = "include")]
            pub include: Vec<Include>,
        }

        impl Validate for SurfaceType {}

        pub mod surface_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct Crgtype {
                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "file")]
                pub file: Option<String>,

                #[yaserde(attribute, rename = "sStart")]
                pub s_start: Option<f64>,

                #[yaserde(attribute, rename = "sEnd")]
                pub s_end: Option<f64>,

                #[yaserde(attribute, rename = "orientation")]
                pub orientation: Option<SurfaceOrientation>,

                #[yaserde(attribute, rename = "mode")]
                pub mode: Option<Mode>,

                #[yaserde(attribute, rename = "purpose")]
                pub purpose: Option<Purpose>,

                #[yaserde(attribute, rename = "sOffset")]
                pub s_offset: Option<f64>,

                #[yaserde(attribute, rename = "tOffset")]
                pub t_offset: Option<f64>,

                #[yaserde(attribute, rename = "zOffset")]
                pub z_offset: Option<f64>,

                #[yaserde(attribute, rename = "zScale")]
                pub z_scale: Option<f64>,

                #[yaserde(attribute, rename = "hOffset")]
                pub h_offset: Option<f64>,
            }

            impl Validate for Crgtype {}
        }

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct RailroadType {
            #[yaserde(rename = "switch")]
            pub switch: Vec<railroad_type::SwitchType>,

            #[yaserde(rename = "userData")]
            pub user_data: Vec<UserData>,

            #[yaserde(rename = "include")]
            pub include: Vec<Include>,
        }

        impl Validate for RailroadType {}

        pub mod railroad_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct SwitchType {
                #[yaserde(rename = "mainTrack")]
                pub main_track: switch_type::MainTrackType,

                #[yaserde(rename = "sideTrack")]
                pub side_track: switch_type::SideTrackType,

                #[yaserde(rename = "partner")]
                pub partner: switch_type::PartnerType,

                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "name")]
                pub name: Option<String>,

                #[yaserde(attribute, rename = "id")]
                pub id: Option<String>,

                #[yaserde(attribute, rename = "position")]
                pub position: Option<Position>,
            }

            impl Validate for SwitchType {}

            pub mod switch_type {
                use super::*;

                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct MainTrackType {
                    #[yaserde(attribute, rename = "id")]
                    pub id: Option<String>,

                    #[yaserde(attribute, rename = "s")]
                    pub s: Option<f64>,

                    #[yaserde(attribute, rename = "dir")]
                    pub dir: Option<Dir>,
                }

                impl Validate for MainTrackType {}
                #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct SideTrackType {
                    #[yaserde(attribute, rename = "id")]
                    pub id: Option<String>,

                    #[yaserde(attribute, rename = "s")]
                    pub s: Option<f64>,

                    #[yaserde(attribute, rename = "dir")]
                    pub dir: Option<Dir>,
                }

                impl Validate for SideTrackType {}
                #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
                #[yaserde()]
                pub struct PartnerType {
                    #[yaserde(attribute, rename = "name")]
                    pub name: Option<String>,

                    #[yaserde(attribute, rename = "id")]
                    pub id: Option<String>,
                }

                impl Validate for PartnerType {}
            }
        }
    }

    #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct ControllerType {
        #[yaserde(rename = "control")]
        pub control: Vec<controller_type::ControlType>,

        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "id")]
        pub id: Option<String>,

        #[yaserde(attribute, rename = "name")]
        pub name: Option<String>,

        #[yaserde(attribute, rename = "sequence")]
        pub sequence: Option<i32>,
    }

    impl Validate for ControllerType {}

    pub mod controller_type {
        use super::*;

        #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct ControlType {
            #[yaserde(rename = "userData")]
            pub user_data: Vec<UserData>,

            #[yaserde(rename = "include")]
            pub include: Vec<Include>,

            #[yaserde(attribute, rename = "signalId")]
            pub signal_id: Option<String>,

            #[yaserde(attribute, rename = "type")]
            pub _type: Option<String>,
        }

        impl Validate for ControlType {}
    }

    #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct JunctionType {
        #[yaserde(rename = "connection")]
        pub connection: Vec<junction_type::ConnectionType>,

        #[yaserde(rename = "priority")]
        pub priority: Vec<junction_type::PriorityType>,

        #[yaserde(rename = "controller")]
        pub controller: Vec<junction_type::ControllerType>,

        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "name")]
        pub name: Option<String>,

        #[yaserde(attribute, rename = "id")]
        pub id: Option<String>,
    }

    impl Validate for JunctionType {}

    pub mod junction_type {
        use super::*;

        #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct ConnectionType {
            #[yaserde(rename = "laneLink")]
            pub lane_link: Vec<connection_type::LaneLinkType>,

            #[yaserde(rename = "userData")]
            pub user_data: Vec<UserData>,

            #[yaserde(rename = "include")]
            pub include: Vec<Include>,

            #[yaserde(attribute, rename = "id")]
            pub id: Option<String>,

            #[yaserde(attribute, rename = "incomingRoad")]
            pub incoming_road: Option<String>,

            #[yaserde(attribute, rename = "connectingRoad")]
            pub connecting_road: Option<String>,

            #[yaserde(attribute, rename = "contactPoint")]
            pub contact_point: Option<ContactPoint>,
        }

        impl Validate for ConnectionType {}

        pub mod connection_type {
            use super::*;

            #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct LaneLinkType {
                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "from")]
                pub from: Option<i32>,

                #[yaserde(attribute, rename = "to")]
                pub to: Option<i32>,
            }

            impl Validate for LaneLinkType {}
        }

        #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct PriorityType {
            #[yaserde(rename = "userData")]
            pub user_data: Vec<UserData>,

            #[yaserde(rename = "include")]
            pub include: Vec<Include>,

            #[yaserde(attribute, rename = "high")]
            pub high: Option<String>,

            #[yaserde(attribute, rename = "low")]
            pub low: Option<String>,
        }

        impl Validate for PriorityType {}
        #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct ControllerType {
            #[yaserde(rename = "userData")]
            pub user_data: Vec<UserData>,

            #[yaserde(rename = "include")]
            pub include: Vec<Include>,

            #[yaserde(attribute, rename = "id")]
            pub id: Option<String>,

            #[yaserde(attribute, rename = "type")]
            pub _type: Option<String>,

            #[yaserde(attribute, rename = "sequence")]
            pub sequence: Option<i32>,
        }

        impl Validate for ControllerType {}
    }

    #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct JunctionGroupType {
        #[yaserde(rename = "junctionReference")]
        pub junction_reference: Vec<junction_group_type::JunctionReferenceType>,

        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "name")]
        pub name: Option<String>,

        #[yaserde(attribute, rename = "id")]
        pub id: Option<String>,

        #[yaserde(attribute, rename = "type")]
        pub _type: Option<JunctionGroupEnum>,
    }

    impl Validate for JunctionGroupType {}

    pub mod junction_group_type {
        use super::*;

        #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct JunctionReferenceType {
            #[yaserde(attribute, rename = "junction")]
            pub junction: Option<String>,
        }

        impl Validate for JunctionReferenceType {}
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct StationType {
        #[yaserde(rename = "platform")]
        pub platform: Vec<station_type::PlatformType>,

        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "name")]
        pub name: Option<String>,

        #[yaserde(attribute, rename = "id")]
        pub id: Option<String>,

        #[yaserde(attribute, rename = "type")]
        pub _type: Option<StationEnum>,
    }

    impl Validate for StationType {}

    pub mod station_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct PlatformType {
            #[yaserde(rename = "segment")]
            pub segment: Vec<platform_type::SegmentType>,

            #[yaserde(attribute, rename = "name")]
            pub name: Option<String>,

            #[yaserde(attribute, rename = "id")]
            pub id: Option<String>,
        }

        impl Validate for PlatformType {}

        pub mod platform_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct SegmentType {
                #[yaserde(rename = "userData")]
                pub user_data: Vec<UserData>,

                #[yaserde(rename = "include")]
                pub include: Vec<Include>,

                #[yaserde(attribute, rename = "roadId")]
                pub road_id: Option<String>,

                #[yaserde(attribute, rename = "sStart")]
                pub s_start: Option<f64>,

                #[yaserde(attribute, rename = "sEnd")]
                pub s_end: Option<f64>,

                #[yaserde(attribute, rename = "side")]
                pub side: Option<Side>,
            }

            impl Validate for SegmentType {}
        }
    }
}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum ElementType {
    #[yaserde(rename = "road")]
    Road,
    #[yaserde(rename = "junction")]
    Junction,
    __Unknown__(String),
}

impl Default for ElementType {
    fn default() -> ElementType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ElementType {}

#[derive(PartialEq, Debug, UtilsUnionSerDe)]
pub enum Max {
    EnumCase0(EnumCaseType0),
    EnumCase1(EnumCaseType1),
    __Unknown__(String),
}

impl Default for Max {
    fn default() -> Max {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Max {}

#[derive(Default, PartialEq, Eq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct EnumCaseType0(pub String);

impl Validate for EnumCaseType0 {}

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct EnumCaseType1(pub xs::Integer);

impl Validate for EnumCaseType1 {
    fn validate(&self) -> Result<(), String> {
        if self.0 < "0".parse::<xs::Integer>().unwrap() {
            return Err(format!("MinInclusive validation error: invalid value of 0! \nExpected: 0 >= 0.\nActual: 0 == {}", self.0));
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum ContactPoint {
    #[yaserde(rename = "start")]
    Start,
    #[yaserde(rename = "end")]
    End,
    __Unknown__(String),
}

impl Default for ContactPoint {
    fn default() -> ContactPoint {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ContactPoint {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum Side {
    #[yaserde(rename = "left")]
    Left,
    #[yaserde(rename = "right")]
    Right,
    __Unknown__(String),
}

impl Default for Side {
    fn default() -> Side {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Side {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum Direction {
    #[yaserde(rename = "same")]
    Same,
    #[yaserde(rename = "opposite")]
    Opposite,
    __Unknown__(String),
}

impl Default for Direction {
    fn default() -> Direction {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Direction {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum RoadType {
    #[yaserde(rename = "unknown")]
    Unknown,
    #[yaserde(rename = "rural")]
    Rural,
    #[yaserde(rename = "motorway")]
    Motorway,
    #[yaserde(rename = "town")]
    Town,
    #[yaserde(rename = "lowSpeed")]
    LowSpeed,
    #[yaserde(rename = "pedestrian")]
    Pedestrian,
    #[yaserde(rename = "bicycle")]
    Bicycle,
    __Unknown__(String),
}

impl Default for RoadType {
    fn default() -> RoadType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for RoadType {}

#[derive(Default, PartialEq, Eq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Unit(pub String);

impl Validate for Unit {}
#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum Prange {
    #[yaserde(rename = "arcLength")]
    ArcLength,
    #[yaserde(rename = "normalized")]
    Normalized,
    __Unknown__(String),
}

impl Default for Prange {
    fn default() -> Prange {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Prange {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum CrossfallSide {
    #[yaserde(rename = "left")]
    Left,
    #[yaserde(rename = "right")]
    Right,
    #[yaserde(rename = "both")]
    Both,
    __Unknown__(String),
}

impl Default for CrossfallSide {
    fn default() -> CrossfallSide {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for CrossfallSide {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum SingleSide {
    #[yaserde(rename = "true")]
    True,
    #[yaserde(rename = "false")]
    False,
    __Unknown__(String),
}

impl Default for SingleSide {
    fn default() -> SingleSide {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for SingleSide {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum LaneType {
    #[yaserde(rename = "none")]
    None,
    #[yaserde(rename = "driving")]
    Driving,
    #[yaserde(rename = "stop")]
    Stop,
    #[yaserde(rename = "shoulder")]
    Shoulder,
    #[yaserde(rename = "biking")]
    Biking,
    #[yaserde(rename = "sidewalk")]
    Sidewalk,
    #[yaserde(rename = "border")]
    Border,
    #[yaserde(rename = "restricted")]
    Restricted,
    #[yaserde(rename = "parking")]
    Parking,
    #[yaserde(rename = "bidirectional")]
    Bidirectional,
    #[yaserde(rename = "median")]
    Median,
    #[yaserde(rename = "special1")]
    Special1,
    #[yaserde(rename = "special2")]
    Special2,
    #[yaserde(rename = "special3")]
    Special3,
    #[yaserde(rename = "roadWorks")]
    RoadWorks,
    #[yaserde(rename = "tram")]
    Tram,
    #[yaserde(rename = "rail")]
    Rail,
    #[yaserde(rename = "entry")]
    Entry,
    #[yaserde(rename = "exit")]
    Exit,
    #[yaserde(rename = "offRamp")]
    OffRamp,
    #[yaserde(rename = "onRamp")]
    OnRamp,
    __Unknown__(String),
}

impl Default for LaneType {
    fn default() -> LaneType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for LaneType {}

#[derive(Default, PartialEq, Eq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct RoadmarkType(pub String);

impl Validate for RoadmarkType {}
#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum Weight {
    #[yaserde(rename = "standard")]
    Standard,
    #[yaserde(rename = "bold")]
    Bold,
    __Unknown__(String),
}

impl Default for Weight {
    fn default() -> Weight {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Weight {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum Color {
    #[yaserde(rename = "standard")]
    Standard,
    #[yaserde(rename = "blue")]
    Blue,
    #[yaserde(rename = "green")]
    Green,
    #[yaserde(rename = "red")]
    Red,
    #[yaserde(rename = "white")]
    White,
    #[yaserde(rename = "yellow")]
    Yellow,
    __Unknown__(String),
}

impl Default for Color {
    fn default() -> Color {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Color {}

#[derive(Default, PartialEq, Eq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Restriction(pub String);

impl Validate for Restriction {}
#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum LaneChange {
    #[yaserde(rename = "increase")]
    Increase,
    #[yaserde(rename = "decrease")]
    Decrease,
    #[yaserde(rename = "both")]
    Both,
    #[yaserde(rename = "none")]
    None,
    __Unknown__(String),
}

impl Default for LaneChange {
    fn default() -> LaneChange {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for LaneChange {}

#[derive(Default, PartialEq, Eq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Rule(pub String);

impl Validate for Rule {}
#[derive(Default, PartialEq, Eq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Orientation(pub String);

impl Validate for Orientation {}
#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum TunnelEnum {
    #[yaserde(rename = "standard")]
    Standard,
    #[yaserde(rename = "underpass")]
    Underpass,
    __Unknown__(String),
}

impl Default for TunnelEnum {
    fn default() -> TunnelEnum {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for TunnelEnum {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum BridgeEnum {
    #[yaserde(rename = "concrete")]
    Concrete,
    #[yaserde(rename = "steel")]
    Steel,
    #[yaserde(rename = "brick")]
    Brick,
    #[yaserde(rename = "wood")]
    Wood,
    __Unknown__(String),
}

impl Default for BridgeEnum {
    fn default() -> BridgeEnum {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for BridgeEnum {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum Access {
    #[yaserde(rename = "all")]
    All,
    #[yaserde(rename = "car")]
    Car,
    #[yaserde(rename = "women")]
    Women,
    #[yaserde(rename = "handicapped")]
    Handicapped,
    #[yaserde(rename = "bus")]
    Bus,
    #[yaserde(rename = "truck")]
    Truck,
    #[yaserde(rename = "electric")]
    Electric,
    #[yaserde(rename = "residents")]
    Residents,
    __Unknown__(String),
}

impl Default for Access {
    fn default() -> Access {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Access {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum ParkingSpacemarkingSide {
    #[yaserde(rename = "front")]
    Front,
    #[yaserde(rename = "rear")]
    Rear,
    #[yaserde(rename = "left")]
    Left,
    #[yaserde(rename = "right")]
    Right,
    __Unknown__(String),
}

impl Default for ParkingSpacemarkingSide {
    fn default() -> ParkingSpacemarkingSide {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ParkingSpacemarkingSide {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum Dynamic {
    #[yaserde(rename = "yes")]
    Yes,
    #[yaserde(rename = "no")]
    No,
    __Unknown__(String),
}

impl Default for Dynamic {
    fn default() -> Dynamic {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Dynamic {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum SurfaceOrientation {
    #[yaserde(rename = "same")]
    Same,
    #[yaserde(rename = "opposite")]
    Opposite,
    __Unknown__(String),
}

impl Default for SurfaceOrientation {
    fn default() -> SurfaceOrientation {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for SurfaceOrientation {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum Mode {
    #[yaserde(rename = "attached")]
    Attached,
    #[yaserde(rename = "attached0")]
    Attached0,
    #[yaserde(rename = "genuine")]
    Genuine,
    __Unknown__(String),
}

impl Default for Mode {
    fn default() -> Mode {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Mode {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum Purpose {
    #[yaserde(rename = "elevation")]
    Elevation,
    #[yaserde(rename = "friction")]
    Friction,
    __Unknown__(String),
}

impl Default for Purpose {
    fn default() -> Purpose {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Purpose {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum Position {
    #[yaserde(rename = "dynamic")]
    Dynamic,
    #[yaserde(rename = "straight")]
    Straight,
    #[yaserde(rename = "turn")]
    Turn,
    __Unknown__(String),
}

impl Default for Position {
    fn default() -> Position {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for Position {}

#[derive(Default, PartialEq, Eq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Dir(pub String);

impl Validate for Dir {}
#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum JunctionGroupEnum {
    #[yaserde(rename = "roundabout")]
    Roundabout,
    #[yaserde(rename = "unknown")]
    Unknown,
    __Unknown__(String),
}

impl Default for JunctionGroupEnum {
    fn default() -> JunctionGroupEnum {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for JunctionGroupEnum {}

#[derive(PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]

pub enum StationEnum {
    #[yaserde(rename = "small")]
    Small,
    #[yaserde(rename = "medium")]
    Medium,
    #[yaserde(rename = "large")]
    Large,
    __Unknown__(String),
}

impl Default for StationEnum {
    fn default() -> StationEnum {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for StationEnum {}

#[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct UserData {
    #[yaserde(attribute, rename = "code")]
    pub code: Option<String>,

    #[yaserde(attribute, rename = "value")]
    pub value: Option<String>,
}

impl Validate for UserData {}

#[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct Include {
    #[yaserde(attribute, rename = "file")]
    pub file: Option<String>,
}

impl Validate for Include {}

#[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct LaneValidity {
    #[yaserde(rename = "userData")]
    pub user_data: Vec<UserData>,

    #[yaserde(rename = "include")]
    pub include: Vec<Include>,

    #[yaserde(attribute, rename = "fromLane")]
    pub from_lane: Option<i32>,

    #[yaserde(attribute, rename = "toLane")]
    pub to_lane: Option<i32>,
}

impl Validate for LaneValidity {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct ParkingSpace {
    #[yaserde(rename = "marking")]
    pub marking: Vec<parking_space::MarkingType>,

    #[yaserde(rename = "userData")]
    pub user_data: Vec<UserData>,

    #[yaserde(rename = "include")]
    pub include: Vec<Include>,

    #[yaserde(attribute, rename = "access")]
    pub access: Option<Access>,

    #[yaserde(attribute, rename = "restrictions")]
    pub restrictions: Option<String>,
}

impl Validate for ParkingSpace {}

pub mod parking_space {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct MarkingType {
        #[yaserde(attribute, rename = "side")]
        pub side: Option<ParkingSpacemarkingSide>,

        #[yaserde(attribute, rename = "type")]
        pub _type: Option<RoadmarkType>,

        #[yaserde(attribute, rename = "width")]
        pub width: Option<f64>,

        #[yaserde(attribute, rename = "color")]
        pub color: Option<Color>,
    }

    impl Validate for MarkingType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct Lane {
    #[yaserde(rename = "link")]
    pub link: lane::LinkType,

    #[yaserde(rename = "width")]
    pub width: Option<lane::WidthType>,

    #[yaserde(rename = "roadMark")]
    pub road_mark: Vec<lane::RoadMarkType>,

    #[yaserde(rename = "material")]
    pub material: Vec<lane::MaterialType>,

    #[yaserde(rename = "visibility")]
    pub visibility: Vec<lane::VisibilityType>,

    #[yaserde(rename = "speed")]
    pub speed: Vec<lane::SpeedType>,

    #[yaserde(rename = "access")]
    pub access: Vec<lane::AccessType>,

    #[yaserde(rename = "height")]
    pub height: Vec<lane::HeightType>,

    #[yaserde(rename = "rule")]
    pub rule: Vec<lane::RuleType>,

    #[yaserde(rename = "userData")]
    pub user_data: Vec<UserData>,

    #[yaserde(rename = "include")]
    pub include: Vec<Include>,

    #[yaserde(attribute, rename = "id")]
    pub id: Option<i32>,

    #[yaserde(attribute, rename = "type")]
    pub _type: Option<LaneType>,

    #[yaserde(attribute, rename = "level")]
    pub level: Option<SingleSide>,
}

impl Validate for Lane {}

pub mod lane {
    use super::*;

    #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct LinkType {
        #[yaserde(rename = "predecessor")]
        pub predecessor: link_type::PredecessorType,

        #[yaserde(rename = "successor")]
        pub successor: link_type::SuccessorType,

        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,
    }

    impl Validate for LinkType {}

    pub mod link_type {
        use super::*;

        #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct PredecessorType {
            #[yaserde(attribute, rename = "id")]
            pub id: Option<i32>,
        }

        impl Validate for PredecessorType {}
        #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct SuccessorType {
            #[yaserde(attribute, rename = "id")]
            pub id: Option<i32>,
        }

        impl Validate for SuccessorType {}
    }

    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]

    pub enum LaneChoice {
        #[yaserde(rename = "width")]
        Width(WidthType),
        #[yaserde(rename = "border")]
        Border(BorderType),
        __Unknown__(String),
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct WidthType {
        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "sOffset")]
        pub s_offset: Option<f64>,

        #[yaserde(attribute, rename = "a")]
        pub a: Option<f64>,

        #[yaserde(attribute, rename = "b")]
        pub b: Option<f64>,

        #[yaserde(attribute, rename = "c")]
        pub c: Option<f64>,

        #[yaserde(attribute, rename = "d")]
        pub d: Option<f64>,
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct BorderType {
        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "sOffset")]
        pub s_offset: Option<f64>,

        #[yaserde(attribute, rename = "a")]
        pub a: Option<f64>,

        #[yaserde(attribute, rename = "b")]
        pub b: Option<f64>,

        #[yaserde(attribute, rename = "c")]
        pub c: Option<f64>,

        #[yaserde(attribute, rename = "d")]
        pub d: Option<f64>,
    }

    impl Default for LaneChoice {
        fn default() -> LaneChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for LaneChoice {}

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct RoadMarkType {
        // TODO
        #[yaserde(rename = "type_2")]
        pub _type_2: road_mark_type::TypeType,

        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "sOffset")]
        pub s_offset: Option<f64>,

        #[yaserde(attribute, rename = "type")]
        pub _type: Option<RoadmarkType>,

        #[yaserde(attribute, rename = "weight")]
        pub weight: Option<Weight>,

        #[yaserde(attribute, rename = "color")]
        pub color: Option<Color>,

        #[yaserde(attribute, rename = "material")]
        pub material: Option<String>,

        #[yaserde(attribute, rename = "width")]
        pub width: Option<f64>,

        #[yaserde(attribute, rename = "laneChange")]
        pub lane_change: Option<LaneChange>,

        #[yaserde(attribute, rename = "height")]
        pub height: Option<f64>,
    }

    impl Validate for RoadMarkType {}

    pub mod road_mark_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct TypeType {
            #[yaserde(rename = "line")]
            pub line: Vec<type_type::LineType>,

            #[yaserde(attribute, rename = "name")]
            pub name: Option<String>,

            #[yaserde(attribute, rename = "width")]
            pub width: Option<f64>,
        }

        impl Validate for TypeType {}

        pub mod type_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct LineType {
                #[yaserde(attribute, rename = "length")]
                pub length: Option<f64>,

                #[yaserde(attribute, rename = "space")]
                pub space: Option<f64>,

                #[yaserde(attribute, rename = "tOffset")]
                pub t_offset: Option<f64>,

                #[yaserde(attribute, rename = "sOffset")]
                pub s_offset: Option<f64>,

                #[yaserde(attribute, rename = "rule")]
                pub rule: Option<Rule>,

                #[yaserde(attribute, rename = "width")]
                pub width: Option<f64>,
            }

            impl Validate for LineType {}
        }
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct MaterialType {
        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "sOffset")]
        pub s_offset: Option<f64>,

        #[yaserde(attribute, rename = "surface")]
        pub surface: Option<String>,

        #[yaserde(attribute, rename = "friction")]
        pub friction: Option<f64>,

        #[yaserde(attribute, rename = "roughness")]
        pub roughness: Option<f64>,
    }

    impl Validate for MaterialType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct VisibilityType {
        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "sOffset")]
        pub s_offset: Option<f64>,

        #[yaserde(attribute, rename = "forward")]
        pub forward: Option<f64>,

        #[yaserde(attribute, rename = "back")]
        pub back: Option<f64>,

        #[yaserde(attribute, rename = "left")]
        pub left: Option<f64>,

        #[yaserde(attribute, rename = "right")]
        pub right: Option<f64>,
    }

    impl Validate for VisibilityType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct SpeedType {
        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "sOffset")]
        pub s_offset: Option<f64>,

        #[yaserde(attribute, rename = "max")]
        pub max: Option<f64>,

        #[yaserde(attribute, rename = "unit")]
        pub unit: Option<Unit>,
    }

    impl Validate for SpeedType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct AccessType {
        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "sOffset")]
        pub s_offset: Option<f64>,

        #[yaserde(attribute, rename = "restriction")]
        pub restriction: Option<Restriction>,
    }

    impl Validate for AccessType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct HeightType {
        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "sOffset")]
        pub s_offset: Option<f64>,

        #[yaserde(attribute, rename = "inner")]
        pub inner: Option<f64>,

        #[yaserde(attribute, rename = "outer")]
        pub outer: Option<f64>,
    }

    impl Validate for HeightType {}
    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct RuleType {
        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "sOffset")]
        pub s_offset: Option<f64>,

        #[yaserde(attribute, rename = "value")]
        pub value: Option<String>,
    }

    impl Validate for RuleType {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct CenterLane {
    #[yaserde(rename = "link")]
    pub link: center_lane::LinkType,

    #[yaserde(rename = "roadMark")]
    pub road_mark: Vec<center_lane::RoadMarkType>,

    #[yaserde(rename = "userData")]
    pub user_data: Vec<UserData>,

    #[yaserde(rename = "include")]
    pub include: Vec<Include>,

    #[yaserde(attribute, rename = "id")]
    pub id: Option<i32>,

    #[yaserde(attribute, rename = "type")]
    pub _type: Option<LaneType>,

    #[yaserde(attribute, rename = "level")]
    pub level: Option<SingleSide>,
}

impl Validate for CenterLane {}

pub mod center_lane {
    use super::*;

    #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct LinkType {
        #[yaserde(rename = "predecessor")]
        pub predecessor: link_type::PredecessorType,

        #[yaserde(rename = "successor")]
        pub successor: link_type::SuccessorType,

        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,
    }

    impl Validate for LinkType {}

    pub mod link_type {
        use super::*;

        #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct PredecessorType {
            #[yaserde(attribute, rename = "id")]
            pub id: Option<i32>,
        }

        impl Validate for PredecessorType {}
        #[derive(Default, PartialEq, Eq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct SuccessorType {
            #[yaserde(attribute, rename = "id")]
            pub id: Option<i32>,
        }

        impl Validate for SuccessorType {}
    }

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct RoadMarkType {
        // TODO
        #[yaserde(rename = "type_2")]
        pub _type_2: road_mark_type::TypeType,

        #[yaserde(rename = "userData")]
        pub user_data: Vec<UserData>,

        #[yaserde(rename = "include")]
        pub include: Vec<Include>,

        #[yaserde(attribute, rename = "sOffset")]
        pub s_offset: Option<f64>,

        #[yaserde(attribute, rename = "type")]
        pub _type: Option<RoadmarkType>,

        #[yaserde(attribute, rename = "weight")]
        pub weight: Option<Weight>,

        #[yaserde(attribute, rename = "color")]
        pub color: Option<Color>,

        #[yaserde(attribute, rename = "material")]
        pub material: Option<String>,

        #[yaserde(attribute, rename = "width")]
        pub width: Option<f64>,

        #[yaserde(attribute, rename = "laneChange")]
        pub lane_change: Option<LaneChange>,

        #[yaserde(attribute, rename = "height")]
        pub height: Option<f64>,
    }

    impl Validate for RoadMarkType {}

    pub mod road_mark_type {
        use super::*;

        #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
        #[yaserde()]
        pub struct TypeType {
            #[yaserde(rename = "line")]
            pub line: Vec<type_type::LineType>,

            #[yaserde(attribute, rename = "name")]
            pub name: Option<String>,

            #[yaserde(attribute, rename = "width")]
            pub width: Option<f64>,
        }

        impl Validate for TypeType {}

        pub mod type_type {
            use super::*;

            #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
            #[yaserde()]
            pub struct LineType {
                #[yaserde(attribute, rename = "length")]
                pub length: Option<f64>,

                #[yaserde(attribute, rename = "space")]
                pub space: Option<f64>,

                #[yaserde(attribute, rename = "tOffset")]
                pub t_offset: Option<f64>,

                #[yaserde(attribute, rename = "sOffset")]
                pub s_offset: Option<f64>,

                #[yaserde(attribute, rename = "rule")]
                pub rule: Option<Rule>,

                #[yaserde(attribute, rename = "width")]
                pub width: Option<f64>,
            }

            impl Validate for LineType {}
        }
    }
}
