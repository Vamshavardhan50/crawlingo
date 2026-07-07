import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import { fileURLToPath } from 'url';
import fs from 'fs';
import nodemailer from 'nodemailer';
import dotenv from 'dotenv';

dotenv.config();

const __dirname = path.dirname(fileURLToPath(import.meta.url));

function waitlistPlugin() {
  const apiHandler = (req, res, next) => {
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET,OPTIONS,PATCH,DELETE,POST,PUT');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

    if (req.url === '/api/waitlist' && req.method === 'OPTIONS') {
      res.statusCode = 200;
      res.end();
      return;
    }

    if (req.url === '/api/waitlist' && req.method === 'POST') {
      let body = '';
      req.on('data', chunk => { body += chunk; });
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

          try {
            const csvPath = path.resolve(__dirname, '../waitlist.csv');
            const exists = fs.existsSync(csvPath);
            const esc = (s) => { if (typeof s !== 'string') return ''; return '"' + s.replace(/"/g, '""') + '"'; };
            const row = `${esc(name)},${esc(email)},${esc(company || '')},${esc(useCase || '')},${esc(new Date().toISOString())}\n`;
            if (!exists) fs.writeFileSync(csvPath, 'Name,Email,Company,Use Case,Signup Date\n' + row);
            else fs.appendFileSync(csvPath, row);
          } catch (e) { console.warn('[waitlist] CSV write failed:', e.message); }

          if (process.env.GOOGLE_SHEET_WEBHOOK_URL) {
            fetch(process.env.GOOGLE_SHEET_WEBHOOK_URL, {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({ name, email, company: company || '', useCase: useCase || '' }),
            }).then(async (r) => { if (!r.ok) console.error('[waitlist] Google Sheets webhook failed:', await r.text()); else console.log('[waitlist] Synced to Google Sheets.'); })
              .catch((e) => console.error('[waitlist] Google Sheets error:', e));
          }

          if (process.env.SMTP_USER && process.env.SMTP_PASS) {
            const transporter = nodemailer.createTransport({ service: 'gmail', auth: { user: process.env.SMTP_USER, pass: process.env.SMTP_PASS } });
            transporter.sendMail({
              from: `"Crawlingo" <${process.env.SMTP_USER}>`,
              to: email,
              subject: 'You are on the Crawlingo Waitlist!',
              html: `<div style="max-width:600px;margin:0 auto;padding:40px 20px;font-family:sans-serif;background:#030512;color:#fff"><div style="background:#090d26;border-radius:16px;padding:40px;text-align:center"><h1 style="margin:0 0 8px;font-size:28px">You're on the list!</h1><div style="font-size:13px;font-family:monospace;color:#563df2;letter-spacing:2px;margin-bottom:30px">Crawlingo Waitlist</div><p style="font-size:15px;line-height:1.6;color:#a5a6b5;text-align:left">Hi <strong style="color:#fff">${name}</strong>,<br/><br/>Thank you for requesting early access to Crawlingo.<br/><br/>We have reserved your spot. You will receive your invitation at <strong style="color:#fff">${email}</strong>.</p></div></div>`,
            }, (err, info) => { if (err) console.error('[waitlist] Email error:', err); else console.log('[waitlist] Email sent:', info.messageId); });
          }

          res.statusCode = 200;
          res.setHeader('Content-Type', 'application/json');
          res.end(JSON.stringify({ success: true, message: 'Signed up successfully!' }));
        } catch (err) {
          console.error('[waitlist] Error:', err);
          res.statusCode = 500;
          res.setHeader('Content-Type', 'application/json');
          res.end(JSON.stringify({ error: 'Server error: ' + err.message }));
        }
      });
    } else {
      next();
    }
  };

  return {
    name: 'waitlist-api',
    configureServer(server) { server.middlewares.use(apiHandler); },
    configurePreviewServer(server) { server.middlewares.use(apiHandler); },
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
