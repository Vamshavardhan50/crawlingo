import nodemailer from 'nodemailer';
import fs from 'fs';
import path from 'path';

export default async function handler(req, res) {
  // Set CORS headers
  res.setHeader('Access-Control-Allow-Credentials', true);
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET,OPTIONS,PATCH,DELETE,POST,PUT');
  res.setHeader(
    'Access-Control-Allow-Headers',
    'X-CSRF-Token, X-Requested-With, Accept, Accept-Version, Content-Length, Content-MD5, Content-Type, Date, X-Api-Version'
  );

  if (req.method === 'OPTIONS') {
    return res.status(200).end();
  }

  if (req.method !== 'POST') {
    return res.status(405).json({ error: 'Method not allowed' });
  }

  try {
    const { name, email, company, useCase } = req.body;

    if (!email || !name) {
      return res.status(400).json({ error: 'Name and email are required.' });
    }

    // Serverless-safe local CSV logger (logged to /tmp, though won't persist across container rebuilds)
    try {
      const csvPath = path.join('/tmp', 'waitlist.csv');
      const exists = fs.existsSync(csvPath);
      const escapeCSV = (str) => {
        if (typeof str !== 'string') return '';
        return '"' + str.replace(/"/g, '""') + '"';
      };
      const newRow = `${escapeCSV(name)},${escapeCSV(email)},${escapeCSV(company || '')},${escapeCSV(useCase || '')},${escapeCSV(new Date().toISOString())}\n`;
      if (!exists) {
        fs.writeFileSync(csvPath, 'Name,Email,Company,Use Case,Signup Date\n' + newRow);
      } else {
        fs.appendFileSync(csvPath, newRow);
      }
    } catch (csvErr) {
      console.warn('[waitlist-api] Local csv logging skipped in serverless environment:', csvErr.message);
    }

    // 1. Forward to Google Sheets Webhook if configured
    if (process.env.GOOGLE_SHEET_WEBHOOK_URL) {
      try {
        const response = await fetch(process.env.GOOGLE_SHEET_WEBHOOK_URL, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({
            name,
            email,
            company: company || '',
            useCase: useCase || '',
          }),
        });

        if (!response.ok) {
          const text = await response.text();
          console.error('[waitlist-api] Google Sheets webhook failed:', text);
        } else {
          console.log('[waitlist-api] Waitlist entry synced to Google Sheets.');
        }
      } catch (sheetErr) {
        console.error('[waitlist-api] Error posting to Google Sheets:', sheetErr);
      }
    } else {
      console.warn('[waitlist-api] GOOGLE_SHEET_WEBHOOK_URL not configured.');
    }

    // 2. Dispatch SMTP confirmation mail if configured
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
                We have reserved your spot in line. We are rolling out private beta access in batches to ensure platform stability. As soon as a slot opens up, we will send your invitation link to this email address (<span class="highlight">${email}</span>).<br/><br/>
                In the meantime, feel free to join our WhatsApp community to connect with other developers and get updates:
              </div>
              <div class="button-container">
                <a class="btn" href="https://github.com/Vamshavardhan50/crawlingo" target="_blank" style="margin-right: 12px; margin-bottom: 12px;">View GitHub Repository</a>
                <a class="btn" href="${process.env.WHATSAPP_GROUP_LINK || '#'}" target="_blank" style="background-color: #25D366; color: #ffffff; margin-bottom: 12px;">Join WhatsApp Group</a>
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

      // Determine robust local path to logo.svg inside Vercel build output
      let logoPath = path.join(process.cwd(), 'public/logo.svg');
      if (!fs.existsSync(logoPath)) {
        logoPath = path.join(process.cwd(), 'docs-v2/public/logo.svg');
      }

      const mailOptions = {
        from: `"Crawlingo" <${process.env.SMTP_USER}>`,
        to: email,
        subject: 'You are on the Crawlingo Waitlist!',
        html: htmlContent,
        attachments: fs.existsSync(logoPath) ? [
          {
            filename: 'logo.svg',
            path: logoPath,
            cid: 'logo_svg'
          }
        ] : []
      };

      // Wrap in Promise to guarantee execution in serverless function before context close
      await new Promise((resolve) => {
        transporter.sendMail(mailOptions, (error, info) => {
          if (error) {
            console.error('[waitlist-api] SMTP sending error:', error);
          } else {
            console.log('[waitlist-api] Waitlist email dispatched:', info.messageId);
          }
          resolve();
        });
      });
    } else {
      console.warn('[waitlist-api] SMTP configuration variables are missing.');
    }

    return res.status(200).json({ success: true, message: 'Signed up successfully!' });
  } catch (err) {
    return res.status(500).json({ error: 'Server error: ' + err.message });
  }
}
