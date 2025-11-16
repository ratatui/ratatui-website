// utils/parse-cargo.js
import { readFile } from "fs/promises";
import toml from "toml";

export async function getRatatuiVersion(filePath: string) {
  const cargoTomlContent = await readFile(filePath, "utf-8");
  const cargoConfig = toml.parse(cargoTomlContent);
  return cargoConfig.dependencies?.ratatui || null;
}
