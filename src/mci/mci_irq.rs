use super::{MCI, consts::*, regs::*};

impl MCI {
    /* Get SDIF controller interrupt mask */
    pub fn interrupt_mask_get(&self, tp: MCIInterruptType) -> u32 {
        let reg = self.config.reg();
        let mut mask = 0;
        if MCIInterruptType::GeneralIntr == tp {
            mask = reg.read_reg::<MCIIntMask>().bits();
        } else if MCIInterruptType::DmaIntr == tp {
            mask = reg.read_reg::<MCIDMACIntEn>().bits();
        }
        mask
    }

    /* Enable/Disable SDIF controller interrupt */
    pub fn interrupt_mask_set(&self, tp: MCIInterruptType, set_mask: u32, enable: bool) {
        let mut mask = self.interrupt_mask_get(tp);
        if enable {
            mask |= set_mask;
        } else {
            mask &= !set_mask;
        }
        let reg = self.config.reg();
        if MCIInterruptType::GeneralIntr == tp {
            reg.write_reg(MCIIntMask::from_bits_truncate(mask));
        } else if MCIInterruptType::DmaIntr == tp {
            reg.write_reg(MCIDMACIntEn::from_bits_truncate(mask));
        }
    }
}
