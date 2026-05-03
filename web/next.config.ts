import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  basePath: "/ai-agent-learning",
  output: "export",
  trailingSlash: true,
  images: {
    unoptimized: true,
  },
};

export default nextConfig;
