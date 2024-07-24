export interface Mirror {
  name: string;
  nameTr: string;
  'name-tr': string;
  loc: string;
  locTr: string;
  'loc-tr': string;
  url: string;
  score?: number;
  downloaded_size?: number;
}

export interface Variant {
  title: string;
  body: string;
  'name-tr': string;
  name: string;
  'description-tr': string;
  description: string;
  retro: boolean;
  squashfs: {
    arch: string;
  }[];
}

export interface Recipe {
  mirrors?: Mirror[];
  variants: Variant[];
}

export interface Device {
  path: string;
  model: string;
  size: number;
}

export interface Partition {
  path?: string;
  parent_path?: string;
  fs_type?: string;
  size: number;
}

export interface SquashfsInfo {
  downloadSize: number;
  instSize: number;
}

export interface PartitionError {
  data: {
    t: string;
  };
}

export interface IsLvmDeviceResp {
  data: boolean;
}

export interface ProgressDetail {
  eta_lo: number;
  eta_hi: number;
  status?:
    | string
    | {
        c: number;
        t: number;
        p: number;
      };
}

export interface Config {
  variant: Variant;
  mirror?: Mirror;
  user: string;
  locale: {
    lang_english: string;
    locale: string;
    title: string;
    lang: string;
    aosc: string;
    inst: string;
    next: string;
    id: string;
    text: string;
    data: string;
  };
  timezone: {
    text: string;
    data: string;
  };
  rtc_as_localtime: boolean;
  device: Device;
  hostname: string;
  mirrors: Mirror[];
  is_offline_install: boolean;
  is_efi: boolean;
  efi_partition?: Partition;
  partition?: Partition;
  swapfile: {
    size: number;
  };
  fullname: string;
  pwd: string;
}

export type RecipeI18n = Record<string, string>;
