use super::*;

#[derive(Debug, PartialEq)]
pub enum PropertyId<'a> {
    Object {
        property: &'a str,
    },
    Object2 {
        property: &'a str,
    },
    Object3 {
        property: &'a str,
    },
    Object4 {
        property: &'a str,
    },
    Object5 {
        property: &'a str,
    },
    Object6 {
        property: &'a str,
    },
    Object7 {
        property: &'a str,
    },
    Object8 {
        property: &'a str,
    },
    Object9 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object10 {
        property: &'a str,
    },
    Object11 {
        property: &'a str,
    },
    Object12 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object13 {
        property: &'a str,
    },
    Object14 {
        property: &'a str,
    },
    Object15 {
        property: &'a str,
    },
    Object16 {
        property: &'a str,
    },
    Object17 {
        property: &'a str,
    },
    Object18 {
        property: &'a str,
    },
    Object19 {
        property: &'a str,
    },
    Object20 {
        property: &'a str,
    },
    Object21 {
        property: &'a str,
    },
    Object22 {
        property: &'a str,
    },
    Object23 {
        property: &'a str,
    },
    Object24 {
        property: &'a str,
    },
    Object25 {
        property: &'a str,
    },
    Object26 {
        property: &'a str,
    },
    Object27 {
        property: &'a str,
    },
    Object28 {
        property: &'a str,
    },
    Object29 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object30 {
        property: &'a str,
    },
    Object31 {
        property: &'a str,
    },
    Object32 {
        property: &'a str,
    },
    Object33 {
        property: &'a str,
    },
    Object34 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object35 {
        property: &'a str,
    },
    Object36 {
        property: &'a str,
    },
    Object37 {
        property: &'a str,
    },
    Object38 {
        property: &'a str,
    },
    Object39 {
        property: &'a str,
    },
    Object40 {
        property: &'a str,
    },
    Object41 {
        property: &'a str,
    },
    Object42 {
        property: &'a str,
    },
    Object43 {
        property: &'a str,
    },
    Object44 {
        property: &'a str,
    },
    Object45 {
        property: &'a str,
    },
    Object46 {
        property: &'a str,
    },
    Object47 {
        property: &'a str,
    },
    Object48 {
        property: &'a str,
    },
    Object49 {
        property: &'a str,
    },
    Object50 {
        property: &'a str,
    },
    Object51 {
        property: &'a str,
    },
    Object52 {
        property: &'a str,
    },
    Object53 {
        property: &'a str,
    },
    Object54 {
        property: &'a str,
    },
    Object55 {
        property: &'a str,
    },
    Object56 {
        property: &'a str,
    },
    Object57 {
        property: &'a str,
    },
    Object58 {
        property: &'a str,
    },
    Object59 {
        property: &'a str,
    },
    Object60 {
        property: &'a str,
    },
    Object61 {
        property: &'a str,
    },
    Object62 {
        property: &'a str,
    },
    Object63 {
        property: &'a str,
    },
    Object64 {
        property: &'a str,
    },
    Object65 {
        property: &'a str,
    },
    Object66 {
        property: &'a str,
    },
    Object67 {
        property: &'a str,
    },
    Object68 {
        property: &'a str,
    },
    Object69 {
        property: &'a str,
    },
    Object70 {
        property: &'a str,
    },
    Object71 {
        property: &'a str,
    },
    Object72 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object73 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object74 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object75 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object76 {
        property: &'a str,
    },
    Object77 {
        property: &'a str,
    },
    Object78 {
        property: &'a str,
    },
    Object79 {
        property: &'a str,
    },
    Object80 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object81 {
        property: &'a str,
    },
    Object82 {
        property: &'a str,
    },
    Object83 {
        property: &'a str,
    },
    Object84 {
        property: &'a str,
    },
    Object85 {
        property: &'a str,
    },
    Object86 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object87 {
        property: &'a str,
    },
    Object88 {
        property: &'a str,
    },
    Object89 {
        property: &'a str,
    },
    Object90 {
        property: &'a str,
    },
    Object91 {
        property: &'a str,
    },
    Object92 {
        property: &'a str,
    },
    Object93 {
        property: &'a str,
    },
    Object94 {
        property: &'a str,
    },
    Object95 {
        property: &'a str,
    },
    Object96 {
        property: &'a str,
    },
    Object97 {
        property: &'a str,
    },
    Object98 {
        property: &'a str,
    },
    Object99 {
        property: &'a str,
    },
    Object100 {
        property: &'a str,
    },
    Object101 {
        property: &'a str,
    },
    Object102 {
        property: &'a str,
    },
    Object103 {
        property: &'a str,
    },
    Object104 {
        property: &'a str,
    },
    Object105 {
        property: &'a str,
    },
    Object106 {
        property: &'a str,
    },
    Object107 {
        property: &'a str,
    },
    Object108 {
        property: &'a str,
    },
    Object109 {
        property: &'a str,
    },
    Object110 {
        property: &'a str,
    },
    Object111 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object112 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object113 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object114 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object115 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object116 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object117 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object118 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object119 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object120 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object121 {
        property: &'a str,
    },
    Object122 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object123 {
        property: &'a str,
    },
    Object124 {
        property: &'a str,
    },
    Object125 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object126 {
        property: &'a str,
    },
    Object127 {
        property: &'a str,
    },
    Object128 {
        property: &'a str,
    },
    Object129 {
        property: &'a str,
    },
    Object130 {
        property: &'a str,
    },
    Object131 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object132 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object133 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object134 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object135 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object136 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object137 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object138 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object139 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object140 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object141 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object142 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object143 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object144 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object145 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object146 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object147 {
        property: &'a str,
    },
    Object148 {
        property: &'a str,
    },
    Object149 {
        property: &'a str,
    },
    Object150 {
        property: &'a str,
    },
    Object151 {
        property: &'a str,
    },
    Object152 {
        property: &'a str,
    },
    Object153 {
        property: &'a str,
    },
    Object154 {
        property: &'a str,
    },
    Object155 {
        property: &'a str,
    },
    Object156 {
        property: &'a str,
    },
    Object157 {
        property: &'a str,
    },
    Object158 {
        property: &'a str,
    },
    Object159 {
        property: &'a str,
    },
    Object160 {
        property: &'a str,
    },
    Object161 {
        property: &'a str,
    },
    Object162 {
        property: &'a str,
    },
    Object163 {
        property: &'a str,
    },
    Object164 {
        property: &'a str,
    },
    Object165 {
        property: &'a str,
    },
    Object166 {
        property: &'a str,
    },
    Object167 {
        property: &'a str,
    },
    Object168 {
        property: &'a str,
    },
    Object169 {
        property: &'a str,
    },
    Object170 {
        property: &'a str,
    },
    Object171 {
        property: &'a str,
    },
    Object172 {
        property: &'a str,
    },
    Object173 {
        property: &'a str,
    },
    Object174 {
        property: &'a str,
    },
    Object175 {
        property: &'a str,
    },
    Object176 {
        property: &'a str,
    },
    Object177 {
        property: &'a str,
    },
    Object178 {
        property: &'a str,
    },
    Object179 {
        property: &'a str,
    },
    Object180 {
        property: &'a str,
    },
    Object181 {
        property: &'a str,
    },
    Object182 {
        property: &'a str,
    },
    Object183 {
        property: &'a str,
    },
    Object184 {
        property: &'a str,
    },
    Object185 {
        property: &'a str,
    },
    Object186 {
        property: &'a str,
    },
    Object187 {
        property: &'a str,
    },
    Object188 {
        property: &'a str,
    },
    Object189 {
        property: &'a str,
    },
    Object190 {
        property: &'a str,
    },
    Object191 {
        property: &'a str,
    },
    Object192 {
        property: &'a str,
    },
    Object193 {
        property: &'a str,
    },
    Object194 {
        property: &'a str,
    },
    Object195 {
        property: &'a str,
    },
    Object196 {
        property: &'a str,
    },
    Object197 {
        property: &'a str,
    },
    Object198 {
        property: &'a str,
    },
    Object199 {
        property: &'a str,
    },
    Object200 {
        property: &'a str,
    },
    Object201 {
        property: &'a str,
    },
    Object202 {
        property: &'a str,
    },
    Object203 {
        property: &'a str,
    },
    Object204 {
        property: &'a str,
    },
    Object205 {
        property: &'a str,
    },
    Object206 {
        property: &'a str,
    },
    Object207 {
        property: &'a str,
    },
    Object208 {
        property: &'a str,
    },
    Object209 {
        property: &'a str,
    },
    Object210 {
        property: &'a str,
    },
    Object211 {
        property: &'a str,
    },
    Object212 {
        property: &'a str,
    },
    Object213 {
        property: &'a str,
    },
    Object214 {
        property: &'a str,
    },
    Object215 {
        property: &'a str,
    },
    Object216 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object217 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object218 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object219 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object220 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object221 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object222 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object223 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object224 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object225 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object226 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object227 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object228 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object229 {
        property: &'a str,
    },
    Object230 {
        property: &'a str,
    },
    Object231 {
        property: &'a str,
    },
    Object232 {
        property: &'a str,
    },
    Object233 {
        property: &'a str,
    },
    Object234 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object235 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object236 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object237 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object238 {
        property: &'a str,
    },
    Object239 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object240 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object241 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object242 {
        property: &'a str,
    },
    Object243 {
        property: &'a str,
    },
    Object244 {
        property: &'a str,
    },
    Object245 {
        property: &'a str,
    },
    Object246 {
        property: &'a str,
    },
    Object247 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object248 {
        property: &'a str,
    },
    Object249 {
        property: &'a str,
    },
    Object250 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object251 {
        property: &'a str,
    },
    Object252 {
        property: &'a str,
    },
    Object253 {
        property: &'a str,
    },
    Object254 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object255 {
        property: &'a str,
    },
    Object256 {
        property: &'a str,
    },
    Object257 {
        property: &'a str,
    },
    Object258 {
        property: &'a str,
    },
    Object259 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object260 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object261 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object262 {
        property: &'a str,
    },
    Object263 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object264 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object265 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object266 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object267 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object268 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object269 {
        property: &'a str,
    },
    Object270 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object271 {
        property: &'a str,
    },
    Object272 {
        property: &'a str,
    },
    Object273 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object274 {
        property: &'a str,
    },
    Object275 {
        property: &'a str,
    },
    Object276 {
        property: &'a str,
    },
    Object277 {
        property: &'a str,
    },
    Object278 {
        property: &'a str,
    },
    Object279 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object280 {
        property: &'a str,
    },
    Object281 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object282 {
        property: &'a str,
    },
    Object283 {
        property: &'a str,
    },
    Object284 {
        property: &'a str,
    },
    Object285 {
        property: &'a str,
    },
    Object286 {
        property: &'a str,
    },
    Object287 {
        property: &'a str,
    },
    Object288 {
        property: &'a str,
    },
    Object289 {
        property: &'a str,
    },
    Object290 {
        property: &'a str,
    },
    Object291 {
        property: &'a str,
    },
    Object292 {
        property: &'a str,
    },
    Object293 {
        property: &'a str,
    },
    Object294 {
        property: &'a str,
    },
    Object295 {
        property: &'a str,
    },
    Object296 {
        property: &'a str,
    },
    Object297 {
        property: &'a str,
    },
    Object298 {
        property: &'a str,
    },
    Object299 {
        property: &'a str,
    },
    Object300 {
        property: &'a str,
    },
    Object301 {
        property: &'a str,
    },
    Object302 {
        property: &'a str,
    },
    Object303 {
        property: &'a str,
    },
    Object304 {
        property: &'a str,
    },
    Object305 {
        property: &'a str,
    },
    Object306 {
        property: &'a str,
    },
    Object307 {
        property: &'a str,
    },
    Object308 {
        property: &'a str,
    },
    Object309 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object310 {
        property: &'a str,
    },
    Object311 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object312 {
        property: &'a str,
    },
    Object313 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object314 {
        property: &'a str,
    },
    Object315 {
        property: &'a str,
    },
    Object316 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object317 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object318 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object319 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object320 {
        property: &'a str,
    },
    Object321 {
        property: &'a str,
    },
    Object322 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object323 {
        property: &'a str,
    },
    Object324 {
        property: &'a str,
    },
    Object325 {
        property: &'a str,
    },
    Object326 {
        property: &'a str,
    },
    Object327 {
        property: &'a str,
    },
    Object328 {
        property: &'a str,
    },
    Object329 {
        property: &'a str,
    },
    Object330 {
        property: &'a str,
    },
    Object331 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object332 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object333 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object334 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object335 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object336 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object337 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object338 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object339 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object340 {
        property: &'a str,
    },
    Object341 {
        property: &'a str,
    },
    Object342 {
        property: &'a str,
    },
    Object343 {
        property: &'a str,
    },
    Object344 {
        property: &'a str,
    },
    Object345 {
        property: &'a str,
    },
    Object346 {
        property: &'a str,
    },
    Object347 {
        property: &'a str,
    },
    Object348 {
        property: &'a str,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object349 {
        property: &'a str,
    },
    Object350 {
        property: &'a str,
    },
}

#[derive(Debug, PartialEq)]
pub enum Prefix {
    None,
    Webkit,
    Moz,
    Ms,
    O,
}

pub type VendorPrefix<'a> = Vec<'a, Box<'a, Prefix>>;

#[derive(Debug, PartialEq)]
pub enum Declaration<'a> {
    Object {
        property: &'a str,
        value: Box<'a, CssColor<'a>>,
    },
    Object2 {
        property: &'a str,
        value: Vec<'a, Box<'a, Image<'a>>>,
    },
    Object3 {
        property: &'a str,
        value: Vec<'a, Box<'a, PositionComponentFor_HorizontalPositionKeyword<'a>>>,
    },
    Object4 {
        property: &'a str,
        value: Vec<'a, Box<'a, PositionComponentFor_VerticalPositionKeyword<'a>>>,
    },
    Object5 {
        property: &'a str,
        value: Vec<'a, Box<'a, BackgroundPosition<'a>>>,
    },
    Object6 {
        property: &'a str,
        value: Vec<'a, Box<'a, BackgroundSize<'a>>>,
    },
    Object7 {
        property: &'a str,
        value: Vec<'a, Box<'a, BackgroundRepeat<'a>>>,
    },
    Object8 {
        property: &'a str,
        value: Vec<'a, Box<'a, BackgroundAttachment>>,
    },
    Object9 {
        property: &'a str,
        value: Vec<'a, Box<'a, BackgroundClip>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object10 {
        property: &'a str,
        value: Vec<'a, Box<'a, BackgroundOrigin>>,
    },
    Object11 {
        property: &'a str,
        value: Vec<'a, Box<'a, Background<'a>>>,
    },
    Object12 {
        property: &'a str,
        value: Vec<'a, Box<'a, BoxShadow<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object13 {
        property: &'a str,
        value: f64,
    },
    Object14 {
        property: &'a str,
        value: Box<'a, CssColor<'a>>,
    },
    Object15 {
        property: &'a str,
        value: Box<'a, Display<'a>>,
    },
    Object16 {
        property: &'a str,
        value: Box<'a, Visibility>,
    },
    Object17 {
        property: &'a str,
        value: Box<'a, Size<'a>>,
    },
    Object18 {
        property: &'a str,
        value: Box<'a, Size<'a>>,
    },
    Object19 {
        property: &'a str,
        value: Box<'a, Size<'a>>,
    },
    Object20 {
        property: &'a str,
        value: Box<'a, Size<'a>>,
    },
    Object21 {
        property: &'a str,
        value: Box<'a, MaxSize<'a>>,
    },
    Object22 {
        property: &'a str,
        value: Box<'a, MaxSize<'a>>,
    },
    Object23 {
        property: &'a str,
        value: Box<'a, Size<'a>>,
    },
    Object24 {
        property: &'a str,
        value: Box<'a, Size<'a>>,
    },
    Object25 {
        property: &'a str,
        value: Box<'a, Size<'a>>,
    },
    Object26 {
        property: &'a str,
        value: Box<'a, Size<'a>>,
    },
    Object27 {
        property: &'a str,
        value: Box<'a, MaxSize<'a>>,
    },
    Object28 {
        property: &'a str,
        value: Box<'a, MaxSize<'a>>,
    },
    Object29 {
        property: &'a str,
        value: Box<'a, BoxSizing>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object30 {
        property: &'a str,
        value: Box<'a, AspectRatio<'a>>,
    },
    Object31 {
        property: &'a str,
        value: Box<'a, Overflow<'a>>,
    },
    Object32 {
        property: &'a str,
        value: Box<'a, OverflowKeyword>,
    },
    Object33 {
        property: &'a str,
        value: Box<'a, OverflowKeyword>,
    },
    Object34 {
        property: &'a str,
        value: Box<'a, TextOverflow>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object35 {
        property: &'a str,
        value: Box<'a, Position2<'a>>,
    },
    Object36 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object37 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object38 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object39 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object40 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object41 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object42 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object43 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object44 {
        property: &'a str,
        value: Box<'a, InsetBlock<'a>>,
    },
    Object45 {
        property: &'a str,
        value: Box<'a, InsetInline<'a>>,
    },
    Object46 {
        property: &'a str,
        value: Box<'a, Inset<'a>>,
    },
    Object47 {
        property: &'a str,
        value: Box<'a, Size2DFor_Length<'a>>,
    },
    Object48 {
        property: &'a str,
        value: Box<'a, CssColor<'a>>,
    },
    Object49 {
        property: &'a str,
        value: Box<'a, CssColor<'a>>,
    },
    Object50 {
        property: &'a str,
        value: Box<'a, CssColor<'a>>,
    },
    Object51 {
        property: &'a str,
        value: Box<'a, CssColor<'a>>,
    },
    Object52 {
        property: &'a str,
        value: Box<'a, CssColor<'a>>,
    },
    Object53 {
        property: &'a str,
        value: Box<'a, CssColor<'a>>,
    },
    Object54 {
        property: &'a str,
        value: Box<'a, CssColor<'a>>,
    },
    Object55 {
        property: &'a str,
        value: Box<'a, CssColor<'a>>,
    },
    Object56 {
        property: &'a str,
        value: Box<'a, LineStyle>,
    },
    Object57 {
        property: &'a str,
        value: Box<'a, LineStyle>,
    },
    Object58 {
        property: &'a str,
        value: Box<'a, LineStyle>,
    },
    Object59 {
        property: &'a str,
        value: Box<'a, LineStyle>,
    },
    Object60 {
        property: &'a str,
        value: Box<'a, LineStyle>,
    },
    Object61 {
        property: &'a str,
        value: Box<'a, LineStyle>,
    },
    Object62 {
        property: &'a str,
        value: Box<'a, LineStyle>,
    },
    Object63 {
        property: &'a str,
        value: Box<'a, LineStyle>,
    },
    Object64 {
        property: &'a str,
        value: Box<'a, BorderSideWidth<'a>>,
    },
    Object65 {
        property: &'a str,
        value: Box<'a, BorderSideWidth<'a>>,
    },
    Object66 {
        property: &'a str,
        value: Box<'a, BorderSideWidth<'a>>,
    },
    Object67 {
        property: &'a str,
        value: Box<'a, BorderSideWidth<'a>>,
    },
    Object68 {
        property: &'a str,
        value: Box<'a, BorderSideWidth<'a>>,
    },
    Object69 {
        property: &'a str,
        value: Box<'a, BorderSideWidth<'a>>,
    },
    Object70 {
        property: &'a str,
        value: Box<'a, BorderSideWidth<'a>>,
    },
    Object71 {
        property: &'a str,
        value: Box<'a, BorderSideWidth<'a>>,
    },
    Object72 {
        property: &'a str,
        value: Box<'a, Size2DFor_DimensionPercentageFor_LengthValue<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object73 {
        property: &'a str,
        value: Box<'a, Size2DFor_DimensionPercentageFor_LengthValue<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object74 {
        property: &'a str,
        value: Box<'a, Size2DFor_DimensionPercentageFor_LengthValue<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object75 {
        property: &'a str,
        value: Box<'a, Size2DFor_DimensionPercentageFor_LengthValue<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object76 {
        property: &'a str,
        value: Box<'a, Size2DFor_DimensionPercentageFor_LengthValue<'a>>,
    },
    Object77 {
        property: &'a str,
        value: Box<'a, Size2DFor_DimensionPercentageFor_LengthValue<'a>>,
    },
    Object78 {
        property: &'a str,
        value: Box<'a, Size2DFor_DimensionPercentageFor_LengthValue<'a>>,
    },
    Object79 {
        property: &'a str,
        value: Box<'a, Size2DFor_DimensionPercentageFor_LengthValue<'a>>,
    },
    Object80 {
        property: &'a str,
        value: Box<'a, BorderRadius<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object81 {
        property: &'a str,
        value: Box<'a, Image<'a>>,
    },
    Object82 {
        property: &'a str,
        value: Box<'a, RectFor_LengthOrNumber<'a>>,
    },
    Object83 {
        property: &'a str,
        value: Box<'a, BorderImageRepeat<'a>>,
    },
    Object84 {
        property: &'a str,
        value: Box<'a, RectFor_BorderImageSideWidth<'a>>,
    },
    Object85 {
        property: &'a str,
        value: Box<'a, BorderImageSlice<'a>>,
    },
    Object86 {
        property: &'a str,
        value: Box<'a, BorderImage<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object87 {
        property: &'a str,
        value: Box<'a, BorderColor<'a>>,
    },
    Object88 {
        property: &'a str,
        value: Box<'a, BorderStyle<'a>>,
    },
    Object89 {
        property: &'a str,
        value: Box<'a, BorderWidth<'a>>,
    },
    Object90 {
        property: &'a str,
        value: Box<'a, BorderBlockColor<'a>>,
    },
    Object91 {
        property: &'a str,
        value: Box<'a, BorderBlockStyle<'a>>,
    },
    Object92 {
        property: &'a str,
        value: Box<'a, BorderBlockWidth<'a>>,
    },
    Object93 {
        property: &'a str,
        value: Box<'a, BorderInlineColor<'a>>,
    },
    Object94 {
        property: &'a str,
        value: Box<'a, BorderInlineStyle<'a>>,
    },
    Object95 {
        property: &'a str,
        value: Box<'a, BorderInlineWidth<'a>>,
    },
    Object96 {
        property: &'a str,
        value: Box<'a, GenericBorderFor_LineStyle<'a>>,
    },
    Object97 {
        property: &'a str,
        value: Box<'a, GenericBorderFor_LineStyle<'a>>,
    },
    Object98 {
        property: &'a str,
        value: Box<'a, GenericBorderFor_LineStyle<'a>>,
    },
    Object99 {
        property: &'a str,
        value: Box<'a, GenericBorderFor_LineStyle<'a>>,
    },
    Object100 {
        property: &'a str,
        value: Box<'a, GenericBorderFor_LineStyle<'a>>,
    },
    Object101 {
        property: &'a str,
        value: Box<'a, GenericBorderFor_LineStyle<'a>>,
    },
    Object102 {
        property: &'a str,
        value: Box<'a, GenericBorderFor_LineStyle<'a>>,
    },
    Object103 {
        property: &'a str,
        value: Box<'a, GenericBorderFor_LineStyle<'a>>,
    },
    Object104 {
        property: &'a str,
        value: Box<'a, GenericBorderFor_LineStyle<'a>>,
    },
    Object105 {
        property: &'a str,
        value: Box<'a, GenericBorderFor_LineStyle<'a>>,
    },
    Object106 {
        property: &'a str,
        value: Box<'a, GenericBorderFor_LineStyle<'a>>,
    },
    Object107 {
        property: &'a str,
        value: Box<'a, GenericBorderFor_OutlineStyleAnd_11<'a>>,
    },
    Object108 {
        property: &'a str,
        value: Box<'a, CssColor<'a>>,
    },
    Object109 {
        property: &'a str,
        value: Box<'a, OutlineStyle<'a>>,
    },
    Object110 {
        property: &'a str,
        value: Box<'a, BorderSideWidth<'a>>,
    },
    Object111 {
        property: &'a str,
        value: Box<'a, FlexDirection>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object112 {
        property: &'a str,
        value: Box<'a, FlexWrap>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object113 {
        property: &'a str,
        value: Box<'a, FlexFlow<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object114 {
        property: &'a str,
        value: f64,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object115 {
        property: &'a str,
        value: f64,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object116 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object117 {
        property: &'a str,
        value: Box<'a, Flex<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object118 {
        property: &'a str,
        value: f64,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object119 {
        property: &'a str,
        value: Box<'a, AlignContent<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object120 {
        property: &'a str,
        value: Box<'a, JustifyContent<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object121 {
        property: &'a str,
        value: Box<'a, PlaceContent<'a>>,
    },
    Object122 {
        property: &'a str,
        value: Box<'a, AlignSelf<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object123 {
        property: &'a str,
        value: Box<'a, JustifySelf<'a>>,
    },
    Object124 {
        property: &'a str,
        value: Box<'a, PlaceSelf<'a>>,
    },
    Object125 {
        property: &'a str,
        value: Box<'a, AlignItems<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object126 {
        property: &'a str,
        value: Box<'a, JustifyItems<'a>>,
    },
    Object127 {
        property: &'a str,
        value: Box<'a, PlaceItems<'a>>,
    },
    Object128 {
        property: &'a str,
        value: Box<'a, GapValue<'a>>,
    },
    Object129 {
        property: &'a str,
        value: Box<'a, GapValue<'a>>,
    },
    Object130 {
        property: &'a str,
        value: Box<'a, Gap<'a>>,
    },
    Object131 {
        property: &'a str,
        value: Box<'a, BoxOrient>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object132 {
        property: &'a str,
        value: Box<'a, BoxDirection>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object133 {
        property: &'a str,
        value: f64,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object134 {
        property: &'a str,
        value: Box<'a, BoxAlign>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object135 {
        property: &'a str,
        value: f64,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object136 {
        property: &'a str,
        value: f64,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object137 {
        property: &'a str,
        value: Box<'a, BoxPack>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object138 {
        property: &'a str,
        value: Box<'a, BoxLines>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object139 {
        property: &'a str,
        value: Box<'a, FlexPack>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object140 {
        property: &'a str,
        value: f64,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object141 {
        property: &'a str,
        value: Box<'a, BoxAlign>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object142 {
        property: &'a str,
        value: Box<'a, FlexItemAlign>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object143 {
        property: &'a str,
        value: Box<'a, FlexLinePack>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object144 {
        property: &'a str,
        value: f64,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object145 {
        property: &'a str,
        value: f64,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object146 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object147 {
        property: &'a str,
        value: Box<'a, TrackSizing<'a>>,
    },
    Object148 {
        property: &'a str,
        value: Box<'a, TrackSizing<'a>>,
    },
    Object149 {
        property: &'a str,
        value: Vec<'a, Box<'a, TrackSize<'a>>>,
    },
    Object150 {
        property: &'a str,
        value: Vec<'a, Box<'a, TrackSize<'a>>>,
    },
    Object151 {
        property: &'a str,
        value: Box<'a, GridAutoFlow<'a>>,
    },
    Object152 {
        property: &'a str,
        value: Box<'a, GridTemplateAreas<'a>>,
    },
    Object153 {
        property: &'a str,
        value: Box<'a, GridTemplate<'a>>,
    },
    Object154 {
        property: &'a str,
        value: Box<'a, Grid<'a>>,
    },
    Object155 {
        property: &'a str,
        value: Box<'a, GridLine<'a>>,
    },
    Object156 {
        property: &'a str,
        value: Box<'a, GridLine<'a>>,
    },
    Object157 {
        property: &'a str,
        value: Box<'a, GridLine<'a>>,
    },
    Object158 {
        property: &'a str,
        value: Box<'a, GridLine<'a>>,
    },
    Object159 {
        property: &'a str,
        value: Box<'a, GridRow<'a>>,
    },
    Object160 {
        property: &'a str,
        value: Box<'a, GridColumn<'a>>,
    },
    Object161 {
        property: &'a str,
        value: Box<'a, GridArea<'a>>,
    },
    Object162 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object163 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object164 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object165 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object166 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object167 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object168 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object169 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object170 {
        property: &'a str,
        value: Box<'a, MarginBlock<'a>>,
    },
    Object171 {
        property: &'a str,
        value: Box<'a, MarginInline<'a>>,
    },
    Object172 {
        property: &'a str,
        value: Box<'a, Margin<'a>>,
    },
    Object173 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object174 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object175 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object176 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object177 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object178 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object179 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object180 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object181 {
        property: &'a str,
        value: Box<'a, PaddingBlock<'a>>,
    },
    Object182 {
        property: &'a str,
        value: Box<'a, PaddingInline<'a>>,
    },
    Object183 {
        property: &'a str,
        value: Box<'a, Padding<'a>>,
    },
    Object184 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object185 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object186 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object187 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object188 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object189 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object190 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object191 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object192 {
        property: &'a str,
        value: Box<'a, ScrollMarginBlock<'a>>,
    },
    Object193 {
        property: &'a str,
        value: Box<'a, ScrollMarginInline<'a>>,
    },
    Object194 {
        property: &'a str,
        value: Box<'a, ScrollMargin<'a>>,
    },
    Object195 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object196 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object197 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object198 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object199 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object200 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object201 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object202 {
        property: &'a str,
        value: Box<'a, LengthPercentageOrAuto<'a>>,
    },
    Object203 {
        property: &'a str,
        value: Box<'a, ScrollPaddingBlock<'a>>,
    },
    Object204 {
        property: &'a str,
        value: Box<'a, ScrollPaddingInline<'a>>,
    },
    Object205 {
        property: &'a str,
        value: Box<'a, ScrollPadding<'a>>,
    },
    Object206 {
        property: &'a str,
        value: Box<'a, FontWeight<'a>>,
    },
    Object207 {
        property: &'a str,
        value: Box<'a, FontSize<'a>>,
    },
    Object208 {
        property: &'a str,
        value: Box<'a, FontStretch<'a>>,
    },
    Object209 {
        property: &'a str,
        value: Vec<'a, Box<'a, FontFamily<'a>>>,
    },
    Object210 {
        property: &'a str,
        value: Box<'a, FontStyle<'a>>,
    },
    Object211 {
        property: &'a str,
        value: Box<'a, FontVariantCaps>,
    },
    Object212 {
        property: &'a str,
        value: Box<'a, LineHeight<'a>>,
    },
    Object213 {
        property: &'a str,
        value: Box<'a, Font<'a>>,
    },
    Object214 {
        property: &'a str,
        value: Box<'a, VerticalAlign<'a>>,
    },
    Object215 {
        property: &'a str,
        value: Box<'a, DashedIdentReference<'a>>,
    },
    Object216 {
        property: &'a str,
        value: Vec<'a, Box<'a, PropertyId<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object217 {
        property: &'a str,
        value: Vec<'a, Box<'a, Time>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object218 {
        property: &'a str,
        value: Vec<'a, Box<'a, Time>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object219 {
        property: &'a str,
        value: Vec<'a, Box<'a, EasingFunction<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object220 {
        property: &'a str,
        value: Vec<'a, Box<'a, Transition<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object221 {
        property: &'a str,
        value: Vec<'a, Box<'a, AnimationName<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object222 {
        property: &'a str,
        value: Vec<'a, Box<'a, Time>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object223 {
        property: &'a str,
        value: Vec<'a, Box<'a, EasingFunction<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object224 {
        property: &'a str,
        value: Vec<'a, Box<'a, AnimationIterationCount>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object225 {
        property: &'a str,
        value: Vec<'a, Box<'a, AnimationDirection>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object226 {
        property: &'a str,
        value: Vec<'a, Box<'a, AnimationPlayState>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object227 {
        property: &'a str,
        value: Vec<'a, Box<'a, Time>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object228 {
        property: &'a str,
        value: Vec<'a, Box<'a, AnimationFillMode>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object229 {
        property: &'a str,
        value: Vec<'a, Box<'a, AnimationComposition>>,
    },
    Object230 {
        property: &'a str,
        value: Vec<'a, Box<'a, AnimationTimeline<'a>>>,
    },
    Object231 {
        property: &'a str,
        value: Vec<'a, Box<'a, AnimationRangeStart<'a>>>,
    },
    Object232 {
        property: &'a str,
        value: Vec<'a, Box<'a, AnimationRangeEnd<'a>>>,
    },
    Object233 {
        property: &'a str,
        value: Vec<'a, Box<'a, AnimationRange<'a>>>,
    },
    Object234 {
        property: &'a str,
        value: Vec<'a, Box<'a, Animation<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object235 {
        property: &'a str,
        value: Vec<'a, Box<'a, Transform<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object236 {
        property: &'a str,
        value: Box<'a, Position<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object237 {
        property: &'a str,
        value: Box<'a, TransformStyle>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object238 {
        property: &'a str,
        value: Box<'a, TransformBox>,
    },
    Object239 {
        property: &'a str,
        value: Box<'a, BackfaceVisibility>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object240 {
        property: &'a str,
        value: Box<'a, Perspective<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object241 {
        property: &'a str,
        value: Box<'a, Position<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object242 {
        property: &'a str,
        value: Box<'a, Translate<'a>>,
    },
    Object243 {
        property: &'a str,
        value: Box<'a, Rotate<'a>>,
    },
    Object244 {
        property: &'a str,
        value: Box<'a, Scale<'a>>,
    },
    Object245 {
        property: &'a str,
        value: Box<'a, TextTransform<'a>>,
    },
    Object246 {
        property: &'a str,
        value: Box<'a, WhiteSpace>,
    },
    Object247 {
        property: &'a str,
        value: Box<'a, LengthOrNumber<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object248 {
        property: &'a str,
        value: Box<'a, WordBreak>,
    },
    Object249 {
        property: &'a str,
        value: Box<'a, LineBreak>,
    },
    Object250 {
        property: &'a str,
        value: Box<'a, Hyphens>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object251 {
        property: &'a str,
        value: Box<'a, OverflowWrap>,
    },
    Object252 {
        property: &'a str,
        value: Box<'a, OverflowWrap>,
    },
    Object253 {
        property: &'a str,
        value: Box<'a, TextAlign>,
    },
    Object254 {
        property: &'a str,
        value: Box<'a, TextAlignLast>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object255 {
        property: &'a str,
        value: Box<'a, TextJustify>,
    },
    Object256 {
        property: &'a str,
        value: Box<'a, Spacing<'a>>,
    },
    Object257 {
        property: &'a str,
        value: Box<'a, Spacing<'a>>,
    },
    Object258 {
        property: &'a str,
        value: Box<'a, TextIndent<'a>>,
    },
    Object259 {
        property: &'a str,
        value: Box<'a, TextDecorationLine<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object260 {
        property: &'a str,
        value: Box<'a, TextDecorationStyle>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object261 {
        property: &'a str,
        value: Box<'a, CssColor<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object262 {
        property: &'a str,
        value: Box<'a, TextDecorationThickness<'a>>,
    },
    Object263 {
        property: &'a str,
        value: Box<'a, TextDecoration<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object264 {
        property: &'a str,
        value: Box<'a, TextDecorationSkipInk>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object265 {
        property: &'a str,
        value: Box<'a, TextEmphasisStyle<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object266 {
        property: &'a str,
        value: Box<'a, CssColor<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object267 {
        property: &'a str,
        value: Box<'a, TextEmphasis<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object268 {
        property: &'a str,
        value: Box<'a, TextEmphasisPosition<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object269 {
        property: &'a str,
        value: Vec<'a, Box<'a, TextShadow<'a>>>,
    },
    Object270 {
        property: &'a str,
        value: Box<'a, TextSizeAdjust>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object271 {
        property: &'a str,
        value: Box<'a, Direction2>,
    },
    Object272 {
        property: &'a str,
        value: Box<'a, UnicodeBidi>,
    },
    Object273 {
        property: &'a str,
        value: Box<'a, BoxDecorationBreak>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object274 {
        property: &'a str,
        value: Box<'a, Resize>,
    },
    Object275 {
        property: &'a str,
        value: Box<'a, Cursor<'a>>,
    },
    Object276 {
        property: &'a str,
        value: Box<'a, ColorOrAuto<'a>>,
    },
    Object277 {
        property: &'a str,
        value: Box<'a, CaretShape>,
    },
    Object278 {
        property: &'a str,
        value: Box<'a, Caret<'a>>,
    },
    Object279 {
        property: &'a str,
        value: Box<'a, UserSelect>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object280 {
        property: &'a str,
        value: Box<'a, ColorOrAuto<'a>>,
    },
    Object281 {
        property: &'a str,
        value: Box<'a, Appearance<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object282 {
        property: &'a str,
        value: Box<'a, ListStyleType<'a>>,
    },
    Object283 {
        property: &'a str,
        value: Box<'a, Image<'a>>,
    },
    Object284 {
        property: &'a str,
        value: Box<'a, ListStylePosition>,
    },
    Object285 {
        property: &'a str,
        value: Box<'a, ListStyle<'a>>,
    },
    Object286 {
        property: &'a str,
        value: Box<'a, MarkerSide>,
    },
    Object287 {
        property: &'a str,
        value: Box<'a, Composes<'a>>,
    },
    Object288 {
        property: &'a str,
        value: Box<'a, SVGPaint<'a>>,
    },
    Object289 {
        property: &'a str,
        value: Box<'a, FillRule>,
    },
    Object290 {
        property: &'a str,
        value: f64,
    },
    Object291 {
        property: &'a str,
        value: Box<'a, SVGPaint<'a>>,
    },
    Object292 {
        property: &'a str,
        value: f64,
    },
    Object293 {
        property: &'a str,
        value: Box<'a, DimensionPercentageFor_LengthValue<'a>>,
    },
    Object294 {
        property: &'a str,
        value: Box<'a, StrokeLinecap>,
    },
    Object295 {
        property: &'a str,
        value: Box<'a, StrokeLinejoin>,
    },
    Object296 {
        property: &'a str,
        value: f64,
    },
    Object297 {
        property: &'a str,
        value: Box<'a, StrokeDasharray<'a>>,
    },
    Object298 {
        property: &'a str,
        value: Box<'a, DimensionPercentageFor_LengthValue<'a>>,
    },
    Object299 {
        property: &'a str,
        value: Box<'a, Marker<'a>>,
    },
    Object300 {
        property: &'a str,
        value: Box<'a, Marker<'a>>,
    },
    Object301 {
        property: &'a str,
        value: Box<'a, Marker<'a>>,
    },
    Object302 {
        property: &'a str,
        value: Box<'a, Marker<'a>>,
    },
    Object303 {
        property: &'a str,
        value: Box<'a, ColorInterpolation>,
    },
    Object304 {
        property: &'a str,
        value: Box<'a, ColorInterpolation>,
    },
    Object305 {
        property: &'a str,
        value: Box<'a, ColorRendering>,
    },
    Object306 {
        property: &'a str,
        value: Box<'a, ShapeRendering>,
    },
    Object307 {
        property: &'a str,
        value: Box<'a, TextRendering>,
    },
    Object308 {
        property: &'a str,
        value: Box<'a, ImageRendering>,
    },
    Object309 {
        property: &'a str,
        value: Box<'a, ClipPath<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object310 {
        property: &'a str,
        value: Box<'a, FillRule>,
    },
    Object311 {
        property: &'a str,
        value: Vec<'a, Box<'a, Image<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object312 {
        property: &'a str,
        value: Vec<'a, Box<'a, MaskMode>>,
    },
    Object313 {
        property: &'a str,
        value: Vec<'a, Box<'a, BackgroundRepeat<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object314 {
        property: &'a str,
        value: Vec<'a, Box<'a, PositionComponentFor_HorizontalPositionKeyword<'a>>>,
    },
    Object315 {
        property: &'a str,
        value: Vec<'a, Box<'a, PositionComponentFor_VerticalPositionKeyword<'a>>>,
    },
    Object316 {
        property: &'a str,
        value: Vec<'a, Box<'a, Position<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object317 {
        property: &'a str,
        value: Vec<'a, Box<'a, MaskClip<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object318 {
        property: &'a str,
        value: Vec<'a, Box<'a, GeometryBox>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object319 {
        property: &'a str,
        value: Vec<'a, Box<'a, BackgroundSize<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object320 {
        property: &'a str,
        value: Vec<'a, Box<'a, MaskComposite>>,
    },
    Object321 {
        property: &'a str,
        value: Box<'a, MaskType>,
    },
    Object322 {
        property: &'a str,
        value: Vec<'a, Box<'a, Mask<'a>>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object323 {
        property: &'a str,
        value: Box<'a, Image<'a>>,
    },
    Object324 {
        property: &'a str,
        value: Box<'a, MaskBorderMode>,
    },
    Object325 {
        property: &'a str,
        value: Box<'a, BorderImageSlice<'a>>,
    },
    Object326 {
        property: &'a str,
        value: Box<'a, RectFor_BorderImageSideWidth<'a>>,
    },
    Object327 {
        property: &'a str,
        value: Box<'a, RectFor_LengthOrNumber<'a>>,
    },
    Object328 {
        property: &'a str,
        value: Box<'a, BorderImageRepeat<'a>>,
    },
    Object329 {
        property: &'a str,
        value: Box<'a, MaskBorder<'a>>,
    },
    Object330 {
        property: &'a str,
        value: Vec<'a, Box<'a, WebKitMaskComposite<'a>>>,
    },
    Object331 {
        property: &'a str,
        value: Vec<'a, Box<'a, WebKitMaskSourceType>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object332 {
        property: &'a str,
        value: Box<'a, BorderImage<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object333 {
        property: &'a str,
        value: Box<'a, Image<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object334 {
        property: &'a str,
        value: Box<'a, BorderImageSlice<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object335 {
        property: &'a str,
        value: Box<'a, RectFor_BorderImageSideWidth<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object336 {
        property: &'a str,
        value: Box<'a, RectFor_LengthOrNumber<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object337 {
        property: &'a str,
        value: Box<'a, BorderImageRepeat<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object338 {
        property: &'a str,
        value: Box<'a, FilterList<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object339 {
        property: &'a str,
        value: Box<'a, FilterList<'a>>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object340 {
        property: &'a str,
        value: Box<'a, ZIndex>,
    },
    Object341 {
        property: &'a str,
        value: Box<'a, ContainerType>,
    },
    Object342 {
        property: &'a str,
        value: Box<'a, ContainerNameList<'a>>,
    },
    Object343 {
        property: &'a str,
        value: Box<'a, Container<'a>>,
    },
    Object344 {
        property: &'a str,
        value: Box<'a, ViewTransitionName<'a>>,
    },
    Object345 {
        property: &'a str,
        value: Box<'a, NoneOrCustomIdentList<'a>>,
    },
    Object346 {
        property: &'a str,
        value: Box<'a, ViewTransitionGroup<'a>>,
    },
    Object347 {
        property: &'a str,
        value: Box<'a, ColorScheme>,
    },
    Object348 {
        property: &'a str,
        value: Box<'a, PrintColorAdjust>,
        vendor_prefix: Box<'a, VendorPrefix<'a>>,
    },
    Object349 {
        property: &'a str,
        value: Box<'a, CSSWideKeyword>,
    },
    Object350 {
        property: &'a str,
        value: Box<'a, UnparsedProperty<'a>>,
    },
    Object351 {
        property: &'a str,
        value: Box<'a, CustomProperty<'a>>,
    },
}
