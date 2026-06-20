import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import fs from 'fs';

function waitlistPlugin() {
  return {
    name: 'waitlist-api',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (req.url === '/api/waitlist' && req.method === 'POST') {
          let body = '';
          req.on('data', chunk => {
            body += chunk;
          });
          req.on('end', () => {
            try {
              const data = JSON.parse(body);
              const { name, email, company, useCase } = data;
              
              if (!email || !name) {
                res.statusCode = 400;
                res.setHeader('Content-Type', 'application/json');
                res.end(JSON.stringify({ error: 'Name and email are required.' }));
                return;
              }

              // Create waitlist.csv in root scraper directory
              const csvPath = path.resolve(__dirname, '../waitlist.csv');
              const exists = fs.existsSync(csvPath);
              
              // Simple CSV escape handler
              const escapeCSV = (str) => {
                if (typeof str !== 'string') return '';
                return '"' + str.replace(/"/g, '""') + '"';
              };

              const newRow = `${escapeCSV(name)},${escapeCSV(email)},${escapeCSV(company || '')},${escapeCSV(useCase || '')},${escapeCSV(new Date().toISOString())}\n`;

              if (!exists) {
                const header = 'Name,Email,Company,Use Case,Signup Date\n';
                fs.writeFileSync(csvPath, header + newRow);
              } else {
                fs.appendFileSync(csvPath, newRow);
              }

              res.statusCode = 200;
              res.setHeader('Content-Type', 'application/json');
              res.end(JSON.stringify({ success: true, message: 'Signed up successfully!' }));
            } catch (err) {
              res.statusCode = 500;
              res.setHeader('Content-Type', 'application/json');
              res.end(JSON.stringify({ error: 'Server error: ' + err.message }));
            }
          });
        } else {
          next();
        }
      });
    }
  };
}

export default defineConfig({
    plugins: [react(), waitlistPlugin()],
    resolve: {
        alias: {
            '@': path.resolve(__dirname, './src'),
        },
    },
    server: {
        port: 3001,
    },
});
