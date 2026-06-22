import { BrowserRouter, Routes, Route } from 'react-router-dom';
import BackgroundGrid from './components/ui/BackgroundGrid';
import Header from './components/layout/Header';
import WaitlistPage from './pages/WaitlistPage';

export default function App() {
  return (
    <BrowserRouter>
      <BackgroundGrid>
        <Header />
        <div className="pt-16">
          <Routes>
            <Route path="/" element={<WaitlistPage />} />
            <Route path="/waitlist" element={<WaitlistPage />} />
          </Routes>
        </div>
      </BackgroundGrid>
    </BrowserRouter>
  );
}
