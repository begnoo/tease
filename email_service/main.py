from email.mime.text import MIMEText
from typing import List
from fastapi import FastAPI
import smtplib
import uvicorn
from pydantic import BaseModel

app = FastAPI()

def send_mail(to, subject, content):
    msg = MIMEText(content)
    msg["Subject"] = subject
    msg['From'] = "tease@tease.com"
    msg['To'] = to[0]
    with smtplib.SMTP("smtp.mailtrap.io", 2525) as server:
        server.login("c3e23bd55db58d", "83ca84be11878c")
        server.sendmail("tease@tease.com", to, msg.as_string())

class CollabEmailRequest(BaseModel):
    user: str
    owner: str
    source: str

@app.post("/collab/")
async def send_collab_mail(request: CollabEmailRequest):
    message = f"""\
    Subject: Collab Alert
    To: {request.user}
    From: tease@tease.com

    {request.owner} asked you to collab on {request.source}, log into your account and check the collab page."""
    send_mail([request.user], "Collab Alert", message)

@app.post("/collab/accept/")
async def send_accept_collab_mail(request: CollabEmailRequest):
    message = f"""\
    Subject: Collab Accepted
    To: {request.owner}
    From: tease@tease.com

    {request.user} accepted to collab with you on {request.source}."""
    send_mail([request.user], "Collab Accepted", message)

@app.post("/collab/reject/")
async def send_collab_mail(request: CollabEmailRequest):
    message = f"""\
    Subject: Collab Alert
    To: {request.user}
    From: tease@tease.com

    {request.owner} rejected to collab with you on {request.source}."""
    send_mail([request.user], "Collab Rejected", message)

@app.get("/")
async def root():
    return {"message": "Hello World"}

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=30000)