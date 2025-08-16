#!/usr/bin/env python3
"""
Guardr Ultimate V2 - AI-Powered Dating Safety Platform
Real APIs + Gemini AI + Advanced OSINT
"""

import os
import sys
import json
import requests
import time
from datetime import datetime
import google.generativeai as genai

class GuardrUltimate:
    def __init__(self):
        # Load all API keys
        self.api_keys = self._load_api_keys()
        
        # Initialize AI
        genai.configure(api_key=self.api_keys['GENERATIVE_LANGUAGE_API_KEY'])
        self.ai_model = genai.GenerativeModel('gemini-1.5-flash')
        
    def _load_api_keys(self):
        """Load API keys from file"""
        keys = {}
        try:
            with open(os.path.expanduser('~/.apikeys.zsh'), 'r') as f:
                for line in f:
                    if 'export ' in line and '=' in line:
                        key = line.strip().replace('export ', '').split('=')[0]
                        value = line.split('=')[1].strip().strip('"\'')
                        keys[key] = value
        except FileNotFoundError:
            print("❌ API keys file not found!")
        return keys
    
    def check_hibp_breaches(self, email):
        """Real Have I Been Pwned API call"""
        url = f"https://haveibeenpwned.com/api/v3/breachedaccount/{email}"
        headers = {
            'hibp-api-key': self.api_keys.get('HIBP_API_KEY', ''),
            'User-Agent': 'Guardr-Dating-Safety-Platform'
        }
        
        try:
            time.sleep(1.5)  # Rate limiting
            response = requests.get(url, headers=headers, timeout=10)
            
            if response.status_code == 200:
                return response.json()
            elif response.status_code == 404:
                return []  # No breaches found
            else:
                print(f"HIBP API Error: {response.status_code}")
                return []
        except Exception as e:
            print(f"HIBP request failed: {e}")
            return []
    
    def check_leak_lookup(self, email):
        """Leak Lookup API for additional breach data"""
        url = "https://leaklookup.com/api/search"
        headers = {
            'X-API-Key': self.api_keys.get('LEAK_LOOKUP_KEY', '')
        }
        
        try:
            data = {'query': email}
            response = requests.post(url, headers=headers, json=data, timeout=10)
            
            if response.status_code == 200:
                return response.json()
            else:
                print(f"Leak Lookup Error: {response.status_code}")
                return {}
        except Exception as e:
            print(f"Leak Lookup failed: {e}")
            return {}
    
    def search_intelligence_x(self, email):
        """Intelligence X search"""
        url = "https://public.intelx.io/intelligent/search"
        headers = {
            'X-Key': self.api_keys.get('INTELX_API_KEY', '')
        }
        
        try:
            data = {
                'term': email,
                'buckets': ['darkweb', 'dumpster', 'leaks'],
                'lookuplevel': 0,
                'maxresults': 100,
                'timeout': 5,
                'datefrom': '',
                'dateto': '',
                'sort': 4,
                'media': 0,
                'terminate': []
            }
            
            response = requests.post(url, headers=headers, json=data, timeout=15)
            if response.status_code == 200:
                return response.json()
            else:
                print(f"IntelX Error: {response.status_code}")
                return {}
        except Exception as e:
            print(f"IntelX search failed: {e}")
            return {}
    
    def ai_risk_analysis(self, email, all_breach_data):
        """AI-powered comprehensive risk analysis"""
        prompt = f"""
        You are an expert cybersecurity analyst specializing in dating safety.
        
        TARGET EMAIL: {email}
        
        BREACH DATA SUMMARY:
        {json.dumps(all_breach_data, indent=2)}
        
        Provide a comprehensive dating safety analysis including:
        
        1. OVERALL RISK SCORE (1-100)
        2. DATING-SPECIFIC VULNERABILITIES
        3. IDENTITY THEFT RISK
        4. SOCIAL ENGINEERING SUSCEPTIBILITY  
        5. ACCOUNT TAKEOVER PROBABILITY
        6. SPECIFIC RED FLAGS FOR DATING
        7. IMMEDIATE PROTECTIVE ACTIONS
        8. LONG-TERM SECURITY RECOMMENDATIONS
        
        Focus on how these breaches specifically impact dating safety and personal security.
        Be thorough but practical. Consider credential stuffing, impersonation, and stalking risks.
        
        Format as detailed JSON with specific risk scores and actionable recommendations.
        """
        
        try:
            response = self.ai_model.generate_content(prompt)
            return self._parse_ai_response(response.text)
        except Exception as e:
            return {"error": f"AI analysis failed: {e}"}
    
    def _parse_ai_response(self, response_text):
        """Parse AI response to extract JSON"""
        try:
            # Find JSON in response
            start = response_text.find('{')
            end = response_text.rfind('}') + 1
            if start != -1 and end != -1:
                json_str = response_text[start:end]
                return json.loads(json_str)
            else:
                return {"analysis": response_text, "parsed": False}
        except json.JSONDecodeError:
            return {"analysis": response_text, "parsed": False}
    
    def comprehensive_investigation(self, email):
        """Full Guardr investigation with all APIs + AI"""
        print(f"🕵️  GUARDR ULTIMATE INVESTIGATION: {email}")
        print("=" * 60)
        
        results = {
            "target": email,
            "timestamp": datetime.now().isoformat(),
            "hibp_breaches": [],
            "leak_lookup_data": {},
            "intelx_data": {},
            "ai_analysis": {},
            "summary": {}
        }
        
        # 1. HIBP Check
        print("🔍 Checking Have I Been Pwned...")
        results["hibp_breaches"] = self.check_hibp_breaches(email)
        print(f"   Found {len(results['hibp_breaches'])} HIBP breaches")
        
        # 2. Leak Lookup
        print("💧 Checking Leak Lookup...")
        results["leak_lookup_data"] = self.check_leak_lookup(email)
        
        # 3. Intelligence X
        print("🕸️  Checking Intelligence X...")
        results["intelx_data"] = self.search_intelligence_x(email)
        
        # 4. AI Analysis
        print("🤖 Generating AI risk analysis...")
        breach_summary = {
            "hibp_breach_count": len(results["hibp_breaches"]),
            "hibp_breaches": results["hibp_breaches"][:5],  # First 5 for AI
            "leak_lookup_found": bool(results["leak_lookup_data"]),
            "intelx_found": bool(results["intelx_data"])
        }
        
        results["ai_analysis"] = self.ai_risk_analysis(email, breach_summary)
        
        # 5. Generate Summary
        results["summary"] = {
            "total_hibp_breaches": len(results["hibp_breaches"]),
            "earliest_breach": min([b.get("BreachDate", "9999-12-31") for b in results["hibp_breaches"]], default="Unknown"),
            "most_recent_breach": max([b.get("BreachDate", "0000-01-01") for b in results["hibp_breaches"]], default="Unknown"),
            "compromised_data_types": list(set([item for breach in results["hibp_breaches"] for item in breach.get("DataClasses", [])])),
            "investigation_complete": True
        }
        
        return results
    
    def print_investigation_report(self, results):
        """Print formatted investigation report"""
        print("\n" + "🛡️  GUARDR INVESTIGATION REPORT".center(80, "="))
        print(f"\n📧 TARGET: {results['target']}")
        print(f"⏰ TIMESTAMP: {results['timestamp']}")
        
        # Summary Stats
        summary = results['summary']
        print(f"\n📊 BREACH SUMMARY:")
        print(f"   💥 Total HIBP Breaches: {summary['total_hibp_breaches']}")
        print(f"   📅 Date Range: {summary['earliest_breach']} to {summary['most_recent_breach']}")
        print(f"   🗂️  Compromised Data Types: {len(summary['compromised_data_types'])}")
        
        # Top Breaches
        if results['hibp_breaches']:
            print(f"\n🔴 TOP BREACHES:")
            for i, breach in enumerate(results['hibp_breaches'][:5], 1):
                print(f"   {i}. {breach.get('Name', 'Unknown')} ({breach.get('BreachDate', 'Unknown')})")
                print(f"      Data: {', '.join(breach.get('DataClasses', []))}")
        
        # AI Analysis
        if results['ai_analysis'] and not results['ai_analysis'].get('error'):
            print(f"\n🤖 AI RISK ASSESSMENT:")
            ai = results['ai_analysis']
            if isinstance(ai, dict) and 'analysis' in ai:
                print(f"   {ai['analysis'][:500]}...")
            else:
                print(f"   Analysis: {str(ai)[:500]}...")
        
        print("\n" + "=" * 80)

def main():
    """Main investigation function"""
    if len(sys.argv) != 2:
        print("Usage: python3 guardr_ultimate_v2.py <email>")
        sys.exit(1)
    
    email = sys.argv[1].strip().lower()
    
    # Initialize Guardr
    guardr = GuardrUltimate()
    
    # Run investigation
    results = guardr.comprehensive_investigation(email)
    
    # Print report
    guardr.print_investigation_report(results)
    
    # Save results
    output_file = f"guardr_report_{email.replace('@', '_at_')}_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
    with open(output_file, 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"\n💾 Full report saved to: {output_file}")

if __name__ == "__main__":
    main()