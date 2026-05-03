import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  basePath: "/rust-ai-agent-learning",
  output: "export",
  trailingSlash: true,
  images: {
    unoptimized: true,
  },
};

export default nextConfig;
