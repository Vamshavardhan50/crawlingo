// app/layout.tsx
import { Layout } from '@/components/layout';

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return <Layout>{children}</Layout>;
}

export const metadata = {
  title: 'Crawlingo Docs',
  description: 'Effortless Self-Healing Web Scraping for the Modern Web',
};
