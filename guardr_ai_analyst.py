#!/usr/bin/env python3
"""
Guardr AI Analyst - Enhanced OSINT with Gemini AI
Uses Google's Generative AI to analyze dating safety risks
"""

import os
import google.generativeai as genai
import json
import requests
from datetime import datetime

class GuardrAIAnalyst:
    def __init__(self):
        # Load API key from environment
        api_key = None
        try:
            with open(os.path.expanduser('~/.apikeys.zsh'), 'r') as f:
                for line in f:
                    if 'GENERATIVE_LANGUAGE_API_KEY' in line and '=' in line:
                        api_key = line.split('=')[1].strip().strip('"\'')
                        break
        except FileNotFoundError:
            # Try environment variable as fallback
            api_key = os.getenv('GENERATIVE_LANGUAGE_API_KEY')
        
        if not api_key:
            raise ValueError("GENERATIVE_LANGUAGE_API_KEY not found in ~/.api_keys.zsh or environment")
        
        genai.configure(api_key=api_key)
        self.model = genai.GenerativeModel('gemini-1.5-flash')
        
    def analyze_dating_profile(self, profile_data):
        """
        AI-powered analysis of dating profile for safety risks
        """
        prompt = f"""
        You are a dating safety expert analyzing a profile for potential red flags.
        
        Profile Data:
        {json.dumps(profile_data, indent=2)}
        
        Analyze this profile for:
        1. Potential catfish indicators
        2. Safety red flags
        3. Authenticity score (1-100)
        4. Specific recommendations
        
        Provide a concise JSON response with:
        - risk_level: LOW/MEDIUM/HIGH
        - authenticity_score: 1-100
        - red_flags: list of concerns
        - recommendations: list of safety tips
        - confidence: how sure you are of this analysis
        """
        
        try:
            response = self.model.generate_content(prompt)
            return self._parse_ai_response(response.text)
        except Exception as e:
            return {"error": f"AI analysis failed: {e}"}
    
    def analyze_breach_data(self, email, breaches):
        """
        AI analysis of breach data for dating safety implications
        """
        prompt = f"""
        You are a cybersecurity expert analyzing data breaches for dating safety.
        
        Email: {email}
        Breaches Found: {len(breaches)}
        
        Breach Details:
        {json.dumps(breaches, indent=2)}
        
        Analyze for dating safety implications:
        1. Password reuse risks
        2. Personal information exposure
        3. Identity theft vulnerability
        4. Dating-specific risks (location, photos, preferences)
        
        Provide JSON response with:
        - overall_risk: LOW/MEDIUM/HIGH/CRITICAL
        - exposed_data_types: list of sensitive data exposed
        - dating_specific_risks: unique risks for dating context
        - immediate_actions: what to do right now
        - long_term_recommendations: ongoing security measures
        """
        
        try:
            response = self.model.generate_content(prompt)
            return self._parse_ai_response(response.text)
        except Exception as e:
            return {"error": f"AI breach analysis failed: {e}"}
    
    def generate_safety_report(self, target_email, all_data):
        """
        Comprehensive AI-generated safety report
        """
        prompt = f"""
        Create a comprehensive dating safety report for someone investigating: {target_email}
        
        Available Data:
        {json.dumps(all_data, indent=2)}
        
        Generate a detailed safety assessment including:
        1. Executive summary of risks
        2. Trust score (1-100)
        3. Key findings and red flags
        4. Specific dating safety recommendations
        5. Questions to ask this person
        6. Warning signs to watch for
        
        Write this as a professional but accessible report for someone's safety.
        Be thorough but not alarmist. Focus on actionable insights.
        """
        
        try:
            response = self.model.generate_content(prompt)
            return response.text
        except Exception as e:
            return f"AI report generation failed: {e}"
    
    def _parse_ai_response(self, response_text):
        """
        Parse AI response, handling JSON extraction
        """
        try:
            # Try to find JSON in the response
            start = response_text.find('{')
            end = response_text.rfind('}') + 1
            if start != -1 and end != -1:
                json_str = response_text[start:end]
                return json.loads(json_str)
            else:
                # If no JSON, return structured response
                return {
                    "analysis": response_text,
                    "parsed": False
                }
        except json.JSONDecodeError:
            return {
                "analysis": response_text,
                "parsed": False,
                "note": "AI response was not in JSON format"
            }

def demo_ai_analysis():
    """
    Demo the AI analysis capabilities
    """
    print("🤖 GUARDR AI ANALYST DEMO")
    print("=" * 50)
    
    ai = GuardrAIAnalyst()
    
    # Demo profile analysis
    demo_profile = {
        "name": "Alex Johnson",
        "age": 28,
        "bio": "Love to travel, entrepreneur, just moved here",
        "photos": ["professional headshot", "travel photo", "gym selfie"],
        "education": "MBA from Stanford",
        "job": "Tech startup founder",
        "location": "Austin, TX",
        "social_media": {
            "instagram": "recent account, few followers",
            "linkedin": "impressive but minimal connections",
            "facebook": "no account found"
        }
    }
    
    print("\n📊 ANALYZING DATING PROFILE...")
    profile_analysis = ai.analyze_dating_profile(demo_profile)
    print(json.dumps(profile_analysis, indent=2))
    
    # Demo breach analysis
    demo_breaches = [
        {
            "Name": "LinkedIn",
            "BreachDate": "2012-05-05",
            "DataClasses": ["Email addresses", "Passwords"]
        },
        {
            "Name": "Adobe",
            "BreachDate": "2013-10-04",
            "DataClasses": ["Email addresses", "Passwords", "Usernames"]
        }
    ]
    
    print("\n🔍 ANALYZING BREACH DATA...")
    breach_analysis = ai.analyze_breach_data("test@example.com", demo_breaches)
    print(json.dumps(breach_analysis, indent=2))
    
    print("\n✅ AI ANALYST READY FOR GUARDR!")

if __name__ == "__main__":
    demo_ai_analysis()