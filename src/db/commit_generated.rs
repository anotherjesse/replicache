#![allow(warnings)]

// automatically generated by the FlatBuffers compiler, do not modify

use std::cmp::Ordering;
use std::mem;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod commit {

    use std::cmp::Ordering;
    use std::mem;

    extern crate flatbuffers;
    use self::flatbuffers::EndianScalar;

    #[allow(non_camel_case_types)]
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub enum MetaTyped {
        NONE = 0,
        IndexChangeMeta = 1,
        LocalMeta = 2,
        SnapshotMeta = 3,
    }

    pub const ENUM_MIN_META_TYPED: u8 = 0;
    pub const ENUM_MAX_META_TYPED: u8 = 3;

    impl<'a> flatbuffers::Follow<'a> for MetaTyped {
        type Inner = Self;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            flatbuffers::read_scalar_at::<Self>(buf, loc)
        }
    }

    impl flatbuffers::EndianScalar for MetaTyped {
        #[inline]
        fn to_little_endian(self) -> Self {
            let n = u8::to_le(self as u8);
            let p = &n as *const u8 as *const MetaTyped;
            unsafe { *p }
        }
        #[inline]
        fn from_little_endian(self) -> Self {
            let n = u8::from_le(self as u8);
            let p = &n as *const u8 as *const MetaTyped;
            unsafe { *p }
        }
    }

    impl flatbuffers::Push for MetaTyped {
        type Output = MetaTyped;
        #[inline]
        fn push(&self, dst: &mut [u8], _rest: &[u8]) {
            flatbuffers::emplace_scalar::<MetaTyped>(dst, *self);
        }
    }

    #[allow(non_camel_case_types)]
    pub const ENUM_VALUES_META_TYPED: [MetaTyped; 4] = [
        MetaTyped::NONE,
        MetaTyped::IndexChangeMeta,
        MetaTyped::LocalMeta,
        MetaTyped::SnapshotMeta,
    ];

    #[allow(non_camel_case_types)]
    pub const ENUM_NAMES_META_TYPED: [&'static str; 4] =
        ["NONE", "IndexChangeMeta", "LocalMeta", "SnapshotMeta"];

    pub fn enum_name_meta_typed(e: MetaTyped) -> &'static str {
        let index = e as u8;
        ENUM_NAMES_META_TYPED[index as usize]
    }

    pub struct MetaTypedUnionTableOffset {}
    pub enum IndexChangeMetaOffset {}
    #[derive(Copy, Clone, Debug, PartialEq)]

    pub struct IndexChangeMeta<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for IndexChangeMeta<'a> {
        type Inner = IndexChangeMeta<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table { buf: buf, loc: loc },
            }
        }
    }

    impl<'a> IndexChangeMeta<'a> {
        #[inline]
        pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            IndexChangeMeta { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args IndexChangeMetaArgs,
        ) -> flatbuffers::WIPOffset<IndexChangeMeta<'bldr>> {
            let mut builder = IndexChangeMetaBuilder::new(_fbb);
            builder.add_last_mutation_id(args.last_mutation_id);
            builder.finish()
        }

        pub const VT_LAST_MUTATION_ID: flatbuffers::VOffsetT = 4;

        #[inline]
        pub fn last_mutation_id(&self) -> u64 {
            self._tab
                .get::<u64>(IndexChangeMeta::VT_LAST_MUTATION_ID, Some(0))
                .unwrap()
        }
    }

    pub struct IndexChangeMetaArgs {
        pub last_mutation_id: u64,
    }
    impl<'a> Default for IndexChangeMetaArgs {
        #[inline]
        fn default() -> Self {
            IndexChangeMetaArgs {
                last_mutation_id: 0,
            }
        }
    }
    pub struct IndexChangeMetaBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> IndexChangeMetaBuilder<'a, 'b> {
        #[inline]
        pub fn add_last_mutation_id(&mut self, last_mutation_id: u64) {
            self.fbb_
                .push_slot::<u64>(IndexChangeMeta::VT_LAST_MUTATION_ID, last_mutation_id, 0);
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        ) -> IndexChangeMetaBuilder<'a, 'b> {
            let start = _fbb.start_table();
            IndexChangeMetaBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<IndexChangeMeta<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    pub enum LocalMetaOffset {}
    #[derive(Copy, Clone, Debug, PartialEq)]

    pub struct LocalMeta<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for LocalMeta<'a> {
        type Inner = LocalMeta<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table { buf: buf, loc: loc },
            }
        }
    }

    impl<'a> LocalMeta<'a> {
        #[inline]
        pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            LocalMeta { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args LocalMetaArgs<'args>,
        ) -> flatbuffers::WIPOffset<LocalMeta<'bldr>> {
            let mut builder = LocalMetaBuilder::new(_fbb);
            builder.add_mutation_id(args.mutation_id);
            if let Some(x) = args.original_hash {
                builder.add_original_hash(x);
            }
            if let Some(x) = args.mutator_args_json {
                builder.add_mutator_args_json(x);
            }
            if let Some(x) = args.mutator_name {
                builder.add_mutator_name(x);
            }
            builder.finish()
        }

        pub const VT_MUTATION_ID: flatbuffers::VOffsetT = 4;
        pub const VT_MUTATOR_NAME: flatbuffers::VOffsetT = 6;
        pub const VT_MUTATOR_ARGS_JSON: flatbuffers::VOffsetT = 8;
        pub const VT_ORIGINAL_HASH: flatbuffers::VOffsetT = 10;

        #[inline]
        pub fn mutation_id(&self) -> u64 {
            self._tab
                .get::<u64>(LocalMeta::VT_MUTATION_ID, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn mutator_name(&self) -> Option<&'a str> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(LocalMeta::VT_MUTATOR_NAME, None)
        }
        #[inline]
        pub fn mutator_args_json(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    LocalMeta::VT_MUTATOR_ARGS_JSON,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn original_hash(&self) -> Option<&'a str> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(LocalMeta::VT_ORIGINAL_HASH, None)
        }
    }

    pub struct LocalMetaArgs<'a> {
        pub mutation_id: u64,
        pub mutator_name: Option<flatbuffers::WIPOffset<&'a str>>,
        pub mutator_args_json: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
        pub original_hash: Option<flatbuffers::WIPOffset<&'a str>>,
    }
    impl<'a> Default for LocalMetaArgs<'a> {
        #[inline]
        fn default() -> Self {
            LocalMetaArgs {
                mutation_id: 0,
                mutator_name: None,
                mutator_args_json: None,
                original_hash: None,
            }
        }
    }
    pub struct LocalMetaBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> LocalMetaBuilder<'a, 'b> {
        #[inline]
        pub fn add_mutation_id(&mut self, mutation_id: u64) {
            self.fbb_
                .push_slot::<u64>(LocalMeta::VT_MUTATION_ID, mutation_id, 0);
        }
        #[inline]
        pub fn add_mutator_name(&mut self, mutator_name: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                LocalMeta::VT_MUTATOR_NAME,
                mutator_name,
            );
        }
        #[inline]
        pub fn add_mutator_args_json(
            &mut self,
            mutator_args_json: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>,
        ) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                LocalMeta::VT_MUTATOR_ARGS_JSON,
                mutator_args_json,
            );
        }
        #[inline]
        pub fn add_original_hash(&mut self, original_hash: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                LocalMeta::VT_ORIGINAL_HASH,
                original_hash,
            );
        }
        #[inline]
        pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> LocalMetaBuilder<'a, 'b> {
            let start = _fbb.start_table();
            LocalMetaBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<LocalMeta<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    pub enum SnapshotMetaOffset {}
    #[derive(Copy, Clone, Debug, PartialEq)]

    pub struct SnapshotMeta<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for SnapshotMeta<'a> {
        type Inner = SnapshotMeta<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table { buf: buf, loc: loc },
            }
        }
    }

    impl<'a> SnapshotMeta<'a> {
        #[inline]
        pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            SnapshotMeta { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args SnapshotMetaArgs<'args>,
        ) -> flatbuffers::WIPOffset<SnapshotMeta<'bldr>> {
            let mut builder = SnapshotMetaBuilder::new(_fbb);
            builder.add_last_mutation_id(args.last_mutation_id);
            if let Some(x) = args.server_state_id {
                builder.add_server_state_id(x);
            }
            builder.finish()
        }

        pub const VT_LAST_MUTATION_ID: flatbuffers::VOffsetT = 4;
        pub const VT_SERVER_STATE_ID: flatbuffers::VOffsetT = 6;

        #[inline]
        pub fn last_mutation_id(&self) -> u64 {
            self._tab
                .get::<u64>(SnapshotMeta::VT_LAST_MUTATION_ID, Some(0))
                .unwrap()
        }
        #[inline]
        pub fn server_state_id(&self) -> Option<&'a str> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(SnapshotMeta::VT_SERVER_STATE_ID, None)
        }
    }

    pub struct SnapshotMetaArgs<'a> {
        pub last_mutation_id: u64,
        pub server_state_id: Option<flatbuffers::WIPOffset<&'a str>>,
    }
    impl<'a> Default for SnapshotMetaArgs<'a> {
        #[inline]
        fn default() -> Self {
            SnapshotMetaArgs {
                last_mutation_id: 0,
                server_state_id: None,
            }
        }
    }
    pub struct SnapshotMetaBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> SnapshotMetaBuilder<'a, 'b> {
        #[inline]
        pub fn add_last_mutation_id(&mut self, last_mutation_id: u64) {
            self.fbb_
                .push_slot::<u64>(SnapshotMeta::VT_LAST_MUTATION_ID, last_mutation_id, 0);
        }
        #[inline]
        pub fn add_server_state_id(&mut self, server_state_id: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                SnapshotMeta::VT_SERVER_STATE_ID,
                server_state_id,
            );
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        ) -> SnapshotMetaBuilder<'a, 'b> {
            let start = _fbb.start_table();
            SnapshotMetaBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<SnapshotMeta<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    pub enum MetaOffset {}
    #[derive(Copy, Clone, Debug, PartialEq)]

    pub struct Meta<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for Meta<'a> {
        type Inner = Meta<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table { buf: buf, loc: loc },
            }
        }
    }

    impl<'a> Meta<'a> {
        #[inline]
        pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            Meta { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args MetaArgs<'args>,
        ) -> flatbuffers::WIPOffset<Meta<'bldr>> {
            let mut builder = MetaBuilder::new(_fbb);
            if let Some(x) = args.typed {
                builder.add_typed(x);
            }
            if let Some(x) = args.checksum {
                builder.add_checksum(x);
            }
            if let Some(x) = args.basis_hash {
                builder.add_basis_hash(x);
            }
            builder.add_typed_type(args.typed_type);
            builder.finish()
        }

        pub const VT_BASIS_HASH: flatbuffers::VOffsetT = 4;
        pub const VT_CHECKSUM: flatbuffers::VOffsetT = 6;
        pub const VT_TYPED_TYPE: flatbuffers::VOffsetT = 8;
        pub const VT_TYPED: flatbuffers::VOffsetT = 10;

        #[inline]
        pub fn basis_hash(&self) -> Option<&'a str> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(Meta::VT_BASIS_HASH, None)
        }
        #[inline]
        pub fn checksum(&self) -> Option<&'a str> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(Meta::VT_CHECKSUM, None)
        }
        #[inline]
        pub fn typed_type(&self) -> MetaTyped {
            self._tab
                .get::<MetaTyped>(Meta::VT_TYPED_TYPE, Some(MetaTyped::NONE))
                .unwrap()
        }
        #[inline]
        pub fn typed(&self) -> Option<flatbuffers::Table<'a>> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Meta::VT_TYPED, None)
        }
        #[inline]
        #[allow(non_snake_case)]
        pub fn typed_as_index_change_meta(&self) -> Option<IndexChangeMeta<'a>> {
            if self.typed_type() == MetaTyped::IndexChangeMeta {
                self.typed().map(|u| IndexChangeMeta::init_from_table(u))
            } else {
                None
            }
        }

        #[inline]
        #[allow(non_snake_case)]
        pub fn typed_as_local_meta(&self) -> Option<LocalMeta<'a>> {
            if self.typed_type() == MetaTyped::LocalMeta {
                self.typed().map(|u| LocalMeta::init_from_table(u))
            } else {
                None
            }
        }

        #[inline]
        #[allow(non_snake_case)]
        pub fn typed_as_snapshot_meta(&self) -> Option<SnapshotMeta<'a>> {
            if self.typed_type() == MetaTyped::SnapshotMeta {
                self.typed().map(|u| SnapshotMeta::init_from_table(u))
            } else {
                None
            }
        }
    }

    pub struct MetaArgs<'a> {
        pub basis_hash: Option<flatbuffers::WIPOffset<&'a str>>,
        pub checksum: Option<flatbuffers::WIPOffset<&'a str>>,
        pub typed_type: MetaTyped,
        pub typed: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
    }
    impl<'a> Default for MetaArgs<'a> {
        #[inline]
        fn default() -> Self {
            MetaArgs {
                basis_hash: None,
                checksum: None,
                typed_type: MetaTyped::NONE,
                typed: None,
            }
        }
    }
    pub struct MetaBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> MetaBuilder<'a, 'b> {
        #[inline]
        pub fn add_basis_hash(&mut self, basis_hash: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Meta::VT_BASIS_HASH, basis_hash);
        }
        #[inline]
        pub fn add_checksum(&mut self, checksum: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Meta::VT_CHECKSUM, checksum);
        }
        #[inline]
        pub fn add_typed_type(&mut self, typed_type: MetaTyped) {
            self.fbb_
                .push_slot::<MetaTyped>(Meta::VT_TYPED_TYPE, typed_type, MetaTyped::NONE);
        }
        #[inline]
        pub fn add_typed(&mut self, typed: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Meta::VT_TYPED, typed);
        }
        #[inline]
        pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MetaBuilder<'a, 'b> {
            let start = _fbb.start_table();
            MetaBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<Meta<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    pub enum IndexDefinitionOffset {}
    #[derive(Copy, Clone, Debug, PartialEq)]

    pub struct IndexDefinition<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for IndexDefinition<'a> {
        type Inner = IndexDefinition<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table { buf: buf, loc: loc },
            }
        }
    }

    impl<'a> IndexDefinition<'a> {
        #[inline]
        pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            IndexDefinition { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args IndexDefinitionArgs<'args>,
        ) -> flatbuffers::WIPOffset<IndexDefinition<'bldr>> {
            let mut builder = IndexDefinitionBuilder::new(_fbb);
            if let Some(x) = args.json_pointer {
                builder.add_json_pointer(x);
            }
            if let Some(x) = args.key_prefix {
                builder.add_key_prefix(x);
            }
            if let Some(x) = args.name {
                builder.add_name(x);
            }
            builder.finish()
        }

        pub const VT_NAME: flatbuffers::VOffsetT = 4;
        pub const VT_KEY_PREFIX: flatbuffers::VOffsetT = 6;
        pub const VT_JSON_POINTER: flatbuffers::VOffsetT = 8;

        #[inline]
        pub fn name(&self) -> Option<&'a str> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(IndexDefinition::VT_NAME, None)
        }
        #[inline]
        pub fn key_prefix(&self) -> Option<&'a [u8]> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(
                    IndexDefinition::VT_KEY_PREFIX,
                    None,
                )
                .map(|v| v.safe_slice())
        }
        #[inline]
        pub fn json_pointer(&self) -> Option<&'a str> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(IndexDefinition::VT_JSON_POINTER, None)
        }
    }

    pub struct IndexDefinitionArgs<'a> {
        pub name: Option<flatbuffers::WIPOffset<&'a str>>,
        pub key_prefix: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
        pub json_pointer: Option<flatbuffers::WIPOffset<&'a str>>,
    }
    impl<'a> Default for IndexDefinitionArgs<'a> {
        #[inline]
        fn default() -> Self {
            IndexDefinitionArgs {
                name: None,
                key_prefix: None,
                json_pointer: None,
            }
        }
    }
    pub struct IndexDefinitionBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> IndexDefinitionBuilder<'a, 'b> {
        #[inline]
        pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(IndexDefinition::VT_NAME, name);
        }
        #[inline]
        pub fn add_key_prefix(
            &mut self,
            key_prefix: flatbuffers::WIPOffset<flatbuffers::Vector<'b, u8>>,
        ) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                IndexDefinition::VT_KEY_PREFIX,
                key_prefix,
            );
        }
        #[inline]
        pub fn add_json_pointer(&mut self, json_pointer: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                IndexDefinition::VT_JSON_POINTER,
                json_pointer,
            );
        }
        #[inline]
        pub fn new(
            _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        ) -> IndexDefinitionBuilder<'a, 'b> {
            let start = _fbb.start_table();
            IndexDefinitionBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<IndexDefinition<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    pub enum IndexRecordOffset {}
    #[derive(Copy, Clone, Debug, PartialEq)]

    pub struct IndexRecord<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for IndexRecord<'a> {
        type Inner = IndexRecord<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table { buf: buf, loc: loc },
            }
        }
    }

    impl<'a> IndexRecord<'a> {
        #[inline]
        pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            IndexRecord { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args IndexRecordArgs<'args>,
        ) -> flatbuffers::WIPOffset<IndexRecord<'bldr>> {
            let mut builder = IndexRecordBuilder::new(_fbb);
            if let Some(x) = args.value_hash {
                builder.add_value_hash(x);
            }
            if let Some(x) = args.definition {
                builder.add_definition(x);
            }
            builder.finish()
        }

        pub const VT_DEFINITION: flatbuffers::VOffsetT = 4;
        pub const VT_VALUE_HASH: flatbuffers::VOffsetT = 6;

        #[inline]
        pub fn definition(&self) -> Option<IndexDefinition<'a>> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<IndexDefinition<'a>>>(
                    IndexRecord::VT_DEFINITION,
                    None,
                )
        }
        #[inline]
        pub fn value_hash(&self) -> Option<&'a str> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(IndexRecord::VT_VALUE_HASH, None)
        }
    }

    pub struct IndexRecordArgs<'a> {
        pub definition: Option<flatbuffers::WIPOffset<IndexDefinition<'a>>>,
        pub value_hash: Option<flatbuffers::WIPOffset<&'a str>>,
    }
    impl<'a> Default for IndexRecordArgs<'a> {
        #[inline]
        fn default() -> Self {
            IndexRecordArgs {
                definition: None,
                value_hash: None,
            }
        }
    }
    pub struct IndexRecordBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> IndexRecordBuilder<'a, 'b> {
        #[inline]
        pub fn add_definition(&mut self, definition: flatbuffers::WIPOffset<IndexDefinition<'b>>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<IndexDefinition>>(
                    IndexRecord::VT_DEFINITION,
                    definition,
                );
        }
        #[inline]
        pub fn add_value_hash(&mut self, value_hash: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                IndexRecord::VT_VALUE_HASH,
                value_hash,
            );
        }
        #[inline]
        pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> IndexRecordBuilder<'a, 'b> {
            let start = _fbb.start_table();
            IndexRecordBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<IndexRecord<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    pub enum CommitOffset {}
    #[derive(Copy, Clone, Debug, PartialEq)]

    pub struct Commit<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for Commit<'a> {
        type Inner = Commit<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table { buf: buf, loc: loc },
            }
        }
    }

    impl<'a> Commit<'a> {
        #[inline]
        pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            Commit { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args CommitArgs<'args>,
        ) -> flatbuffers::WIPOffset<Commit<'bldr>> {
            let mut builder = CommitBuilder::new(_fbb);
            if let Some(x) = args.indexes {
                builder.add_indexes(x);
            }
            if let Some(x) = args.value_hash {
                builder.add_value_hash(x);
            }
            if let Some(x) = args.meta {
                builder.add_meta(x);
            }
            builder.finish()
        }

        pub const VT_META: flatbuffers::VOffsetT = 4;
        pub const VT_VALUE_HASH: flatbuffers::VOffsetT = 6;
        pub const VT_INDEXES: flatbuffers::VOffsetT = 8;

        #[inline]
        pub fn meta(&self) -> Option<Meta<'a>> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<Meta<'a>>>(Commit::VT_META, None)
        }
        #[inline]
        pub fn value_hash(&self) -> Option<&'a str> {
            self._tab
                .get::<flatbuffers::ForwardsUOffset<&str>>(Commit::VT_VALUE_HASH, None)
        }
        #[inline]
        pub fn indexes(
            &self,
        ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<IndexRecord<'a>>>>
        {
            self._tab.get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<flatbuffers::ForwardsUOffset<IndexRecord<'a>>>,
            >>(Commit::VT_INDEXES, None)
        }
    }

    pub struct CommitArgs<'a> {
        pub meta: Option<flatbuffers::WIPOffset<Meta<'a>>>,
        pub value_hash: Option<flatbuffers::WIPOffset<&'a str>>,
        pub indexes: Option<
            flatbuffers::WIPOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<IndexRecord<'a>>>,
            >,
        >,
    }
    impl<'a> Default for CommitArgs<'a> {
        #[inline]
        fn default() -> Self {
            CommitArgs {
                meta: None,
                value_hash: None,
                indexes: None,
            }
        }
    }
    pub struct CommitBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> CommitBuilder<'a, 'b> {
        #[inline]
        pub fn add_meta(&mut self, meta: flatbuffers::WIPOffset<Meta<'b>>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<Meta>>(Commit::VT_META, meta);
        }
        #[inline]
        pub fn add_value_hash(&mut self, value_hash: flatbuffers::WIPOffset<&'b str>) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Commit::VT_VALUE_HASH, value_hash);
        }
        #[inline]
        pub fn add_indexes(
            &mut self,
            indexes: flatbuffers::WIPOffset<
                flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<IndexRecord<'b>>>,
            >,
        ) {
            self.fbb_
                .push_slot_always::<flatbuffers::WIPOffset<_>>(Commit::VT_INDEXES, indexes);
        }
        #[inline]
        pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> CommitBuilder<'a, 'b> {
            let start = _fbb.start_table();
            CommitBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<Commit<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    #[inline]
    pub fn get_root_as_commit<'a>(buf: &'a [u8]) -> Commit<'a> {
        flatbuffers::get_root::<Commit<'a>>(buf)
    }

    #[inline]
    pub fn get_size_prefixed_root_as_commit<'a>(buf: &'a [u8]) -> Commit<'a> {
        flatbuffers::get_size_prefixed_root::<Commit<'a>>(buf)
    }

    #[inline]
    pub fn finish_commit_buffer<'a, 'b>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        root: flatbuffers::WIPOffset<Commit<'a>>,
    ) {
        fbb.finish(root, None);
    }

    #[inline]
    pub fn finish_size_prefixed_commit_buffer<'a, 'b>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        root: flatbuffers::WIPOffset<Commit<'a>>,
    ) {
        fbb.finish_size_prefixed(root, None);
    }
} // pub mod commit
