import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import fs from 'fs';
import nodemailer from 'nodemailer';
import dotenv from 'dotenv';

dotenv.config();

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

              // Send waitlist confirmation email if SMTP is configured
              if (process.env.SMTP_USER && process.env.SMTP_PASS) {
                const transporter = nodemailer.createTransport({
                  service: 'gmail',
                  auth: {
                    user: process.env.SMTP_USER,
                    pass: process.env.SMTP_PASS,
                  },
                });

                const htmlContent = `
                  <!DOCTYPE html>
                  <html>
                  <head>
                    <meta charset="utf-8">
                    <style>
                      body {
                        background-color: #030512;
                        color: #ffffff;
                        font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
                        margin: 0;
                        padding: 0;
                      }
                      .container {
                        max-width: 600px;
                        margin: 0 auto;
                        padding: 40px 20px;
                      }
                      .card {
                        background-color: #090d26;
                        border: 1px solid rgba(255, 255, 255, 0.08);
                        border-radius: 16px;
                        padding: 40px;
                        text-align: center;
                        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
                      }
                      .logo-container {
                        margin-bottom: 24px;
                        display: inline-block;
                      }
                      .logo-img {
                        width: 64px;
                        height: 64px;
                      }
                      .title {
                        font-size: 28px;
                        font-weight: 800;
                        letter-spacing: -0.5px;
                        margin-top: 0;
                        margin-bottom: 8px;
                        color: #ffffff;
                      }
                      .subtitle {
                        font-size: 13px;
                        font-family: monospace;
                        color: #563df2;
                        text-transform: uppercase;
                        letter-spacing: 2px;
                        margin-bottom: 30px;
                      }
                      .message {
                        font-size: 15px;
                        line-height: 1.6;
                        color: #a5a6b5;
                        margin-bottom: 30px;
                        text-align: left;
                      }
                      .highlight {
                        color: #ffffff;
                        font-weight: 600;
                      }
                      .button-container {
                        margin: 35px 0;
                      }
                      .btn {
                        background-color: #ffffff;
                        color: #000000;
                        text-decoration: none;
                        padding: 12px 30px;
                        font-size: 14px;
                        font-weight: 600;
                        border-radius: 8px;
                        display: inline-block;
                      }
                      .footer {
                        margin-top: 40px;
                        font-size: 11px;
                        color: #525464;
                        line-height: 1.5;
                      }
                    </style>
                  </head>
                  <body>
                    <div class="container">
                      <div class="card">
                        <div class="logo-container">
                          <img class="logo-img" src="cid:logo_svg" alt="Crawlingo Logo" />
                        </div>
                        <h1 class="title">You're on the list!</h1>
                        <div class="subtitle">Crawlingo Waitlist</div>
                        <div class="message">
                          Hi <span class="highlight">${name}</span>,<br/><br/>
                          Thank you for requesting early access to <strong>Crawlingo</strong>, the self-healing scraping framework designed to survive website DOM changes.<br/><br/>
                          We have reserved your spot in line. We are rolling out private beta access in batches to ensure platform stability. As soon as a slot opens up, we will send your invitation link to this email address (<span class="highlight">${email}</span>).
                        </div>
                        <div class="button-container">
                          <a class="btn" href="https://github.com/Vamshavardhan50/crawlingo" target="_blank">View GitHub Repository</a>
                        </div>
                        <div class="footer">
                          You received this because you signed up for the Crawlingo Waitlist.<br/>
                          &copy; ${new Date().getFullYear()} Crawlingo. All rights reserved.
                        </div>
                      </div>
                    </div>
                  </body>
                  </html>
                `;

                const mailOptions = {
                  from: `"Crawlingo" <${process.env.SMTP_USER}>`,
                  to: email,
                  subject: 'You are on the Crawlingo Waitlist!',
                  html: htmlContent,
                  attachments: [
                    {
                      filename: 'logo.svg',
                      path: path.resolve(__dirname, 'public/logo.svg'),
                      cid: 'logo_svg'
                    }
                  ]
                };

                transporter.sendMail(mailOptions, (error, info) => {
                  if (error) {
                    console.error('[waitlist-api] Error sending confirmation email:', error);
                  } else {
                    console.log('[waitlist-api] Waitlist email dispatched successfully:', info.messageId);
                  }
                });
              } else {
                console.warn('[waitlist-api] SMTP credentials not set. Confirmation email skipped.');
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
