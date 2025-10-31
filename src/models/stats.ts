export interface GitHubRepo {
  stars: number;
  forks: number;
}

export interface CratesIoResponse {
  crate: {
    downloads: number;
  };
}

export interface CratesIoReverseDepsResponse {
  dependencies: Array<{
    crate: {
      name: string;
    };
  }>;
  meta: {
    total: number;
  };
}
