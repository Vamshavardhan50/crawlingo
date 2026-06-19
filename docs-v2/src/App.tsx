import { BrowserRouter, Routes, Route } from 'react-router-dom';
import DocsLayout from './components/layout/DocsLayout';
import IntroductionPage from './pages/IntroductionPage';
import GettingStartedPage from './pages/GettingStartedPage';
import ArchitecturePage from './pages/ArchitecturePage';
import FeaturesPage from './pages/FeaturesPage';
import APIReferencePage from './pages/APIReferencePage';
import SDKPage from './pages/SDKPage';
import IntegrationsPage from './pages/IntegrationsPage';
import AdvancedPage from './pages/AdvancedPage';
import TroubleshootingPage from './pages/TroubleshootingPage';
import ChangelogPage from './pages/ChangelogPage';
import RoadmapPage from './pages/RoadmapPage';

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<DocsLayout />}>
          <Route path="/" element={<IntroductionPage />} />
          <Route path="/getting-started" element={<GettingStartedPage />} />
          <Route path="/architecture" element={<ArchitecturePage />} />
          <Route path="/features" element={<FeaturesPage />} />
          <Route path="/api-reference" element={<APIReferencePage />} />
          <Route path="/sdk" element={<SDKPage />} />
          <Route path="/integrations" element={<IntegrationsPage />} />
          <Route path="/advanced" element={<AdvancedPage />} />
          <Route path="/troubleshooting" element={<TroubleshootingPage />} />
          <Route path="/changelog" element={<ChangelogPage />} />
          <Route path="/roadmap" element={<RoadmapPage />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}