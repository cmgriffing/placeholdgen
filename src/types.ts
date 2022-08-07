export interface Site {
  name: string;
  id: string;
  localPath: string;
}

export interface BaseSiteImage {
  id: string;
  uploaded: boolean;
}

export interface ExternalSiteImage extends BaseSiteImage {
  uploaded: false;
  sourceUrl: string;
}

export interface LocalSiteImage extends BaseSiteImage {
  uploaded: true;
  originalFileName: string;
}

export type SiteImage = ExternalSiteImage | LocalSiteImage;

export interface Collection {
  collectionId: string;
  name: string;
  images: SiteImage[];
}
