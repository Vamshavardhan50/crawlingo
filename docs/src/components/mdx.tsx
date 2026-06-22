import defaultMdxComponents from 'fumadocs-ui/mdx';
import type { MDXComponents } from 'mdx/types';
import { BentoGrid, BentoGridItem } from './BentoGrid';
import HoverBorderCard from './HoverBorderCard';
import SelectorSimulator from './SelectorSimulator';
import EagleLogo from './EagleLogo';
import DemoSlideshow from './DemoSlideshow';

export function getMDXComponents(components?: MDXComponents) {
  return {
    ...defaultMdxComponents,
    BentoGrid,
    BentoGridItem,
    HoverBorderCard,
    SelectorSimulator,
    EagleLogo,
    DemoSlideshow,
    ...components,
  } satisfies MDXComponents;
}

export const useMDXComponents = getMDXComponents;

declare global {
  type MDXProvidedComponents = ReturnType<typeof getMDXComponents>;
}


