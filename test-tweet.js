require('dotenv').config();
const { TwitterApi } = require('twitter-api-v2');

const xClient = new TwitterApi({
  appKey: process.env.X_API_KEY,
  appSecret: process.env.X_API_SECRET,
  accessToken: process.env.X_ACCESS_TOKEN,
  accessSecret: process.env.X_ACCESS_SECRET,
}).readWrite;

async function test() {
  try {
    const res = await xClient.v2.tweet('⚡ @ZKResolver Bot System Check: zkTLS Prover active on Base network.');
    console.log('✅ TEST TWEET SUCCESSFUL! Tweet ID:', res.data.id);
  } catch (err) {
    console.error('❌ X API ERROR:', err);
  }
}

test();