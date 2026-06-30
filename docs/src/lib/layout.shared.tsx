import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';
import { gitConfig } from './shared';
import EagleLogo from '@/components/EagleLogo';

export function baseOptions(): BaseLayoutProps {
  return {
    nav: {
      title: <EagleLogo size="sm" variant="full" />,
    },
    githubUrl: `https://github.com/${gitConfig.user}/${gitConfig.repo}`,
    searchToggle: {
      enabled: true,
    },
    themeSwitch: {
      enabled: true,
    },
  };
}
