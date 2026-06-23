import { createMDX } from 'fumadocs-mdx/next';

const withMDX = createMDX();

/** @type {import('next').NextConfig} */
const isVercel = process.env.VERCEL === '1';

const config = {
  reactStrictMode: true,
  output: isVercel ? undefined : 'export',
  trailingSlash: true,
};


export default withMDX(config);
