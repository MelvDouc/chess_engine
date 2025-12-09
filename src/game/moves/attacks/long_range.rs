use crate::{
    game::board::{NB_SQUARES, directions as dirs},
    macros::const_while,
};

struct MagicEntry {
    mask: u64,
    magic: u64,
    shift: usize,
    offset: usize,
}

impl MagicEntry {
    pub(super) const fn mask(&self) -> u64 {
        self.mask
    }

    pub(super) const fn get_index(&self, occ: u64) -> usize {
        let index = occ.wrapping_mul(self.magic) >> self.shift;
        index as usize + self.offset
    }
}

type DirArray = [usize; 4];

const fn square_attacks(sq: usize, dirs: DirArray, occ: u64) -> u64 {
    let mut attacks = 0;

    const_while!(i, 0, 4, {
        let dir = dirs[i];
        let mut ray = dirs::ray_of(sq, dir);

        if ray & occ != 0 {
            let occupied_sq = dirs::first_occupied_square(ray & occ, dir);
            ray &= !dirs::ray_of(occupied_sq, dir);
        }

        attacks |= ray;
    });

    attacks
}

const fn fill_table(table: &mut [u64], dirs: DirArray, magic: &MagicEntry, sq: usize) {
    let mask = magic.mask();
    let mut occ = 0;

    loop {
        let index = magic.get_index(occ);
        table[index] = square_attacks(sq, dirs, occ);
        occ = occ.wrapping_sub(mask) & mask;

        if occ == 0 {
            break;
        }
    }
}

macro_rules! create_table {
    ($dirs: expr, $magics: expr, $table_size: expr) => {{
        let mut table = [0u64; $table_size];

        const_while!(sq, 0, NB_SQUARES, {
            let entry = &$magics[sq];
            fill_table(&mut table, $dirs, entry, sq);
        });

        table
    }};
}

macro_rules! long_range_attacks {
    ($table: expr, $magic: expr, $occ: expr) => {
        $table[$magic.get_index($occ & $magic.mask())]
    };
}

pub(super) const fn bishop_attacks(sq: usize, occ: u64) -> u64 {
    const TABLE_SIZE: usize = 5248;

    const DIRECTIONS: DirArray = [
        dirs::NORTH_EAST,
        dirs::NORTH_WEST,
        dirs::SOUTH_EAST,
        dirs::SOUTH_WEST,
    ];

    const MAGICS: [MagicEntry; NB_SQUARES] = [
        MagicEntry {
            mask: 18049651735527936,
            magic: 18172736983597684,
            shift: 58,
            offset: 0,
        },
        MagicEntry {
            mask: 70506452091904,
            magic: 9803228089688399888,
            shift: 59,
            offset: 64,
        },
        MagicEntry {
            mask: 275415828992,
            magic: 13520153593577537,
            shift: 59,
            offset: 96,
        },
        MagicEntry {
            mask: 1075975168,
            magic: 325389477567529040,
            shift: 59,
            offset: 128,
        },
        MagicEntry {
            mask: 38021120,
            magic: 4653271075586064,
            shift: 59,
            offset: 160,
        },
        MagicEntry {
            mask: 8657588224,
            magic: 5067719037157379,
            shift: 59,
            offset: 192,
        },
        MagicEntry {
            mask: 2216338399232,
            magic: 291613679059994752,
            shift: 59,
            offset: 224,
        },
        MagicEntry {
            mask: 567382630219776,
            magic: 2315132244207346178,
            shift: 58,
            offset: 256,
        },
        MagicEntry {
            mask: 9024825867763712,
            magic: 1736146461311373580,
            shift: 59,
            offset: 320,
        },
        MagicEntry {
            mask: 18049651735527424,
            magic: 1603305691630600768,
            shift: 59,
            offset: 352,
        },
        MagicEntry {
            mask: 70506452221952,
            magic: 581096223843329,
            shift: 59,
            offset: 384,
        },
        MagicEntry {
            mask: 275449643008,
            magic: 45893636852285568,
            shift: 59,
            offset: 416,
        },
        MagicEntry {
            mask: 9733406720,
            magic: 1243138787847180329,
            shift: 59,
            offset: 448,
        },
        MagicEntry {
            mask: 2216342585344,
            magic: 1153029875295060100,
            shift: 59,
            offset: 480,
        },
        MagicEntry {
            mask: 567382630203392,
            magic: 9227877839877062656,
            shift: 59,
            offset: 512,
        },
        MagicEntry {
            mask: 1134765260406784,
            magic: 72339621589558784,
            shift: 59,
            offset: 544,
        },
        MagicEntry {
            mask: 4512412933816832,
            magic: 441355244130287872,
            shift: 59,
            offset: 576,
        },
        MagicEntry {
            mask: 9024825867633664,
            magic: 2314850217125544450,
            shift: 59,
            offset: 608,
        },
        MagicEntry {
            mask: 18049651768822272,
            magic: 40532431545638944,
            shift: 57,
            offset: 640,
        },
        MagicEntry {
            mask: 70515108615168,
            magic: 1155173512760017944,
            shift: 57,
            offset: 768,
        },
        MagicEntry {
            mask: 2491752130560,
            magic: 2308377471835179010,
            shift: 57,
            offset: 896,
        },
        MagicEntry {
            mask: 567383701868544,
            magic: 36099191549763594,
            shift: 57,
            offset: 1024,
        },
        MagicEntry {
            mask: 1134765256220672,
            magic: 1128135504694808,
            shift: 59,
            offset: 1152,
        },
        MagicEntry {
            mask: 2269530512441344,
            magic: 9234982880265113600,
            shift: 59,
            offset: 1184,
        },
        MagicEntry {
            mask: 2256206450263040,
            magic: 4612829511061524576,
            shift: 59,
            offset: 1216,
        },
        MagicEntry {
            mask: 4512412900526080,
            magic: 1238516295201261825,
            shift: 59,
            offset: 1248,
        },
        MagicEntry {
            mask: 9024834391117824,
            magic: 10137843187975168,
            shift: 57,
            offset: 1280,
        },
        MagicEntry {
            mask: 18051867805491712,
            magic: 16286150948638838992,
            shift: 55,
            offset: 1408,
        },
        MagicEntry {
            mask: 637888545440768,
            magic: 2414527543025672,
            shift: 55,
            offset: 1920,
        },
        MagicEntry {
            mask: 1135039602493440,
            magic: 72345666906527878,
            shift: 57,
            offset: 2432,
        },
        MagicEntry {
            mask: 2269529440784384,
            magic: 613056916750401796,
            shift: 59,
            offset: 2560,
        },
        MagicEntry {
            mask: 4539058881568768,
            magic: 3468618408988377344,
            shift: 59,
            offset: 2592,
        },
        MagicEntry {
            mask: 1128098963916800,
            magic: 73337701591683072,
            shift: 59,
            offset: 2624,
        },
        MagicEntry {
            mask: 2256197927833600,
            magic: 77689602976133122,
            shift: 59,
            offset: 2656,
        },
        MagicEntry {
            mask: 4514594912477184,
            magic: 989629184541249,
            shift: 57,
            offset: 2688,
        },
        MagicEntry {
            mask: 9592139778506752,
            magic: 1603283668531937408,
            shift: 55,
            offset: 2816,
        },
        MagicEntry {
            mask: 19184279556981248,
            magic: 10403315706178404640,
            shift: 55,
            offset: 3328,
        },
        MagicEntry {
            mask: 2339762086609920,
            magic: 9223706584742494465,
            shift: 57,
            offset: 3840,
        },
        MagicEntry {
            mask: 4538784537380864,
            magic: 2162330357805088920,
            shift: 59,
            offset: 3968,
        },
        MagicEntry {
            mask: 9077569074761728,
            magic: 422783697847296,
            shift: 59,
            offset: 4000,
        },
        MagicEntry {
            mask: 562958610993152,
            magic: 2308114808611287041,
            shift: 59,
            offset: 4032,
        },
        MagicEntry {
            mask: 1125917221986304,
            magic: 2306285081880166960,
            shift: 59,
            offset: 4064,
        },
        MagicEntry {
            mask: 2814792987328512,
            magic: 2333146391762702336,
            shift: 57,
            offset: 4096,
        },
        MagicEntry {
            mask: 5629586008178688,
            magic: 1164251697325801988,
            shift: 57,
            offset: 4224,
        },
        MagicEntry {
            mask: 11259172008099840,
            magic: 21994695320577,
            shift: 57,
            offset: 4352,
        },
        MagicEntry {
            mask: 22518341868716544,
            magic: 2505215272999748097,
            shift: 57,
            offset: 4480,
        },
        MagicEntry {
            mask: 9007336962655232,
            magic: 290499778897511936,
            shift: 59,
            offset: 4608,
        },
        MagicEntry {
            mask: 18014673925310464,
            magic: 301268367048960,
            shift: 59,
            offset: 4640,
        },
        MagicEntry {
            mask: 2216338399232,
            magic: 108867079209812097,
            shift: 59,
            offset: 4672,
        },
        MagicEntry {
            mask: 4432676798464,
            magic: 568451940876296,
            shift: 59,
            offset: 4704,
        },
        MagicEntry {
            mask: 11064376819712,
            magic: 1729982625860813072,
            shift: 59,
            offset: 4736,
        },
        MagicEntry {
            mask: 22137335185408,
            magic: 72058195879135249,
            shift: 59,
            offset: 4768,
        },
        MagicEntry {
            mask: 44272556441600,
            magic: 4505044918861824,
            shift: 59,
            offset: 4800,
        },
        MagicEntry {
            mask: 87995357200384,
            magic: 2287001684541760,
            shift: 59,
            offset: 4832,
        },
        MagicEntry {
            mask: 35253226045952,
            magic: 27057625041616896,
            shift: 59,
            offset: 4864,
        },
        MagicEntry {
            mask: 70506452091904,
            magic: 74362188168571200,
            shift: 59,
            offset: 4896,
        },
        MagicEntry {
            mask: 567382630219776,
            magic: 144401619713328256,
            shift: 58,
            offset: 4928,
        },
        MagicEntry {
            mask: 1134765260406784,
            magic: 801650371687878656,
            shift: 59,
            offset: 4992,
        },
        MagicEntry {
            mask: 2832480465846272,
            magic: 144115196670117888,
            shift: 59,
            offset: 5024,
        },
        MagicEntry {
            mask: 5667157807464448,
            magic: 4611686027069851649,
            shift: 59,
            offset: 5056,
        },
        MagicEntry {
            mask: 11333774449049600,
            magic: 1441160814827537536,
            shift: 59,
            offset: 5088,
        },
        MagicEntry {
            mask: 22526811443298304,
            magic: 216212639546671361,
            shift: 59,
            offset: 5120,
        },
        MagicEntry {
            mask: 9024825867763712,
            magic: 35193029660808,
            shift: 59,
            offset: 5152,
        },
        MagicEntry {
            mask: 18049651735527936,
            magic: 18025395840453252,
            shift: 58,
            offset: 5184,
        },
    ];

    const TABLE: [u64; TABLE_SIZE] = create_table!(DIRECTIONS, MAGICS, TABLE_SIZE);

    long_range_attacks!(TABLE, MAGICS[sq], occ)
}

pub(super) const fn rook_attacks(sq: usize, occ: u64) -> u64 {
    const TABLE_SIZE: usize = 102400;

    const DIRECTIONS: DirArray = [dirs::NORTH, dirs::SOUTH, dirs::EAST, dirs::WEST];

    const MAGICS: [MagicEntry; NB_SQUARES] = [
        MagicEntry {
            mask: 282578800148862,
            magic: 36029621654816593,
            shift: 52,
            offset: 0,
        },
        MagicEntry {
            mask: 565157600297596,
            magic: 9385501941804568576,
            shift: 53,
            offset: 4096,
        },
        MagicEntry {
            mask: 1130315200595066,
            magic: 180153336043937792,
            shift: 53,
            offset: 6144,
        },
        MagicEntry {
            mask: 2260630401190006,
            magic: 9835870382345719809,
            shift: 53,
            offset: 8192,
        },
        MagicEntry {
            mask: 4521260802379886,
            magic: 15060045954315079700,
            shift: 53,
            offset: 10240,
        },
        MagicEntry {
            mask: 9042521604759646,
            magic: 144117387367613448,
            shift: 53,
            offset: 12288,
        },
        MagicEntry {
            mask: 18085043209519166,
            magic: 288239189693481218,
            shift: 53,
            offset: 14336,
        },
        MagicEntry {
            mask: 36170086419038334,
            magic: 72057732626518272,
            shift: 52,
            offset: 16384,
        },
        MagicEntry {
            mask: 282578800180736,
            magic: 864831866484555904,
            shift: 53,
            offset: 20480,
        },
        MagicEntry {
            mask: 565157600328704,
            magic: 2305913383328681984,
            shift: 54,
            offset: 22528,
        },
        MagicEntry {
            mask: 1130315200625152,
            magic: 648659221310607362,
            shift: 54,
            offset: 23552,
        },
        MagicEntry {
            mask: 2260630401218048,
            magic: 108227680582766592,
            shift: 54,
            offset: 24576,
        },
        MagicEntry {
            mask: 4521260802403840,
            magic: 4612389723049558144,
            shift: 54,
            offset: 25600,
        },
        MagicEntry {
            mask: 9042521604775424,
            magic: 18577700650487832,
            shift: 54,
            offset: 26624,
        },
        MagicEntry {
            mask: 18085043209518592,
            magic: 36169693429502464,
            shift: 54,
            offset: 27648,
        },
        MagicEntry {
            mask: 36170086419037696,
            magic: 5911114118397954,
            shift: 53,
            offset: 28672,
        },
        MagicEntry {
            mask: 282578808340736,
            magic: 7205796237440729106,
            shift: 53,
            offset: 30720,
        },
        MagicEntry {
            mask: 565157608292864,
            magic: 9223513874441977860,
            shift: 54,
            offset: 32768,
        },
        MagicEntry {
            mask: 1130315208328192,
            magic: 1412872712225088,
            shift: 54,
            offset: 33792,
        },
        MagicEntry {
            mask: 2260630408398848,
            magic: 2252349779218434,
            shift: 54,
            offset: 34816,
        },
        MagicEntry {
            mask: 4521260808540160,
            magic: 37383697363528,
            shift: 54,
            offset: 35840,
        },
        MagicEntry {
            mask: 9042521608822784,
            magic: 6939203700493910273,
            shift: 54,
            offset: 36864,
        },
        MagicEntry {
            mask: 18085043209388032,
            magic: 9223376434918633008,
            shift: 54,
            offset: 37888,
        },
        MagicEntry {
            mask: 36170086418907136,
            magic: 3464431396767957060,
            shift: 53,
            offset: 38912,
        },
        MagicEntry {
            mask: 282580897300736,
            magic: 9225694207560155169,
            shift: 53,
            offset: 40960,
        },
        MagicEntry {
            mask: 565159647117824,
            magic: 4620693357272760448,
            shift: 54,
            offset: 43008,
        },
        MagicEntry {
            mask: 1130317180306432,
            magic: 1441172775776550978,
            shift: 54,
            offset: 44032,
        },
        MagicEntry {
            mask: 2260632246683648,
            magic: 19149171822961152,
            shift: 54,
            offset: 45056,
        },
        MagicEntry {
            mask: 4521262379438080,
            magic: 577736187939390080,
            shift: 54,
            offset: 46080,
        },
        MagicEntry {
            mask: 9042522644946944,
            magic: 721704041465250816,
            shift: 54,
            offset: 47104,
        },
        MagicEntry {
            mask: 18085043175964672,
            magic: 565166156646416,
            shift: 54,
            offset: 48128,
        },
        MagicEntry {
            mask: 36170086385483776,
            magic: 4665729772301600516,
            shift: 53,
            offset: 49152,
        },
        MagicEntry {
            mask: 283115671060736,
            magic: 324261647076040706,
            shift: 53,
            offset: 51200,
        },
        MagicEntry {
            mask: 565681586307584,
            magic: 3453020595560960,
            shift: 54,
            offset: 53248,
        },
        MagicEntry {
            mask: 1130822006735872,
            magic: 1175440673130700800,
            shift: 54,
            offset: 54272,
        },
        MagicEntry {
            mask: 2261102847592448,
            magic: 864973290777620480,
            shift: 54,
            offset: 55296,
        },
        MagicEntry {
            mask: 4521664529305600,
            magic: 1157565876090708992,
            shift: 54,
            offset: 56320,
        },
        MagicEntry {
            mask: 9042787892731904,
            magic: 2315413231469855816,
            shift: 54,
            offset: 57344,
        },
        MagicEntry {
            mask: 18085034619584512,
            magic: 8070456038463148088,
            shift: 54,
            offset: 58368,
        },
        MagicEntry {
            mask: 36170077829103616,
            magic: 277058945284,
            shift: 53,
            offset: 59392,
        },
        MagicEntry {
            mask: 420017753620736,
            magic: 2305914486068477984,
            shift: 53,
            offset: 61440,
        },
        MagicEntry {
            mask: 699298018886144,
            magic: 76561478743834624,
            shift: 54,
            offset: 63488,
        },
        MagicEntry {
            mask: 1260057572672512,
            magic: 292734130532147200,
            shift: 54,
            offset: 64512,
        },
        MagicEntry {
            mask: 2381576680245248,
            magic: 1301540365475577888,
            shift: 54,
            offset: 65536,
        },
        MagicEntry {
            mask: 4624614895390720,
            magic: 4899935086411055108,
            shift: 54,
            offset: 66560,
        },
        MagicEntry {
            mask: 9110691325681664,
            magic: 4621821316679368832,
            shift: 54,
            offset: 67584,
        },
        MagicEntry {
            mask: 18082844186263552,
            magic: 5354927360775028872,
            shift: 54,
            offset: 68608,
        },
        MagicEntry {
            mask: 36167887395782656,
            magic: 9529665467120222209,
            shift: 53,
            offset: 69632,
        },
        MagicEntry {
            mask: 35466950888980736,
            magic: 36029072184214144,
            shift: 53,
            offset: 71680,
        },
        MagicEntry {
            mask: 34905104758997504,
            magic: 141855254782208,
            shift: 54,
            offset: 73728,
        },
        MagicEntry {
            mask: 34344362452452352,
            magic: 2314851342880870656,
            shift: 54,
            offset: 74752,
        },
        MagicEntry {
            mask: 33222877839362048,
            magic: 580542693460224,
            shift: 54,
            offset: 75776,
        },
        MagicEntry {
            mask: 30979908613181440,
            magic: 614741366337044544,
            shift: 54,
            offset: 76800,
        },
        MagicEntry {
            mask: 26493970160820224,
            magic: 288793360733635072,
            shift: 54,
            offset: 77824,
        },
        MagicEntry {
            mask: 17522093256097792,
            magic: 576478358582469632,
            shift: 54,
            offset: 78848,
        },
        MagicEntry {
            mask: 35607136465616896,
            magic: 144396665233663744,
            shift: 53,
            offset: 79872,
        },
        MagicEntry {
            mask: 9079539427579068672,
            magic: 2450098935317987401,
            shift: 52,
            offset: 81920,
        },
        MagicEntry {
            mask: 8935706818303361536,
            magic: 2269822570266753,
            shift: 53,
            offset: 86016,
        },
        MagicEntry {
            mask: 8792156787827803136,
            magic: 1441293752656330818,
            shift: 53,
            offset: 88064,
        },
        MagicEntry {
            mask: 8505056726876686336,
            magic: 9367325806593,
            shift: 53,
            offset: 90112,
        },
        MagicEntry {
            mask: 7930856604974452736,
            magic: 72620578687688706,
            shift: 53,
            offset: 92160,
        },
        MagicEntry {
            mask: 6782456361169985536,
            magic: 1688923481835522,
            shift: 53,
            offset: 94208,
        },
        MagicEntry {
            mask: 4485655873561051136,
            magic: 562984321810786,
            shift: 53,
            offset: 96256,
        },
        MagicEntry {
            mask: 9115426935197958144,
            magic: 1207106816310285314,
            shift: 52,
            offset: 98304,
        },
    ];

    #[allow(long_running_const_eval)]
    const TABLE: [u64; TABLE_SIZE] = create_table!(DIRECTIONS, MAGICS, TABLE_SIZE);

    long_range_attacks!(TABLE, MAGICS[sq], occ)
}
