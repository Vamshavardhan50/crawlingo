// next.config.mjs
export default {
  experimental: {
    turbo: true,
    optimizeCss: true,
    optimizePackageImports: ["@radix-ui/react", "lucide-react", "framer-motion"]
  },
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "raw.githubusercontent.com",
        pathname: "/**",
      },
      {
        protocol: "https",
        hostname: "crawlingo.dev",
        pathname: "/**",
      },
      {
        protocol: "https",
        hostname: "www.npmjs.com",
        pathname: "/**",
      },
      {
        protocol: "https",
        hostname: "img.shields.io",
        pathname: "/**",
      },
      {
        protocol: "https",
        hostname: "star-history.com",
        pathname: "/**",
      },
    ],
  },
  compiler: {
    removeConsole: process.env.NODE_ENV === "production",
  },
};