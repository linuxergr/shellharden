/*
 * Copyright 2016 - 2019 Andreas Nordal
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::situation::Situation;
use crate::situation::Transition;
use crate::situation::WhatNow;
use crate::situation::flush;

use crate::microparsers::predlen;

pub struct SitUntilByte {
	pub until: u8,
	pub color: u32,
}

impl Situation for SitUntilByte {
	fn whatnow(&mut self, horizon: &[u8], _is_horizon_lengthenable: bool) -> WhatNow {
		let len = predlen(|x| x != self.until, horizon);
		if len < horizon.len() {
			WhatNow{tri: Transition::Pop, pre: len, len: 1, alt: None}
		} else {
			flush(len)
		}
	}
	fn get_color(&self) -> u32 {
		self.color
	}
}
