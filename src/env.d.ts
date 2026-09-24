/// <reference path="../.astro/types.d.ts" />
/// <reference types="astro/client" />

// Starlight's compiled package no longer exposes its internal virtual-module declarations.
declare module "virtual:starlight/user-config" {
  const config: import("@astrojs/starlight/types").StarlightConfig;
  export default config;
}
