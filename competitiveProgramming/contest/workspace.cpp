/****************************************************
*                                                  *
        Author : siddhantDeshwal                 
        Date   : 04th January 2025, 21:39           
*                                                  *
****************************************************/

#include <algorithm>
#include <chrono>
#include <climits>
#include <cmath>
#include <cstring>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;
using ll = signed long long int;

// ;(
struct custom_hash
{
  static uint64_t
  splitmix64 (uint64_t x)
  {
    x += 0x9e3779b97f4a7c15;
    x = (x ^ (x >> 30)) * 0xbf58476d1ce4e5b9;
    x = (x ^ (x >> 27)) * 0x94d049bb133111eb;
    return x ^ (x >> 31);
  }

  size_t
  operator() (uint64_t x) const
  {
    static const uint64_t FIXED_RANDOM
        = chrono::steady_clock::now ().time_since_epoch ().count ();
    return splitmix64 (x + FIXED_RANDOM);
  }
};

const ll M = 1e9 + 7;

// Macros
#define ff first
#define ss second
#define pb push_back
#define mp make_pair
#define fl(i, n) for (int i = 0; i < n; i++)
#define in(v) fl (i, v.size ()) cin >> v[i];
#define py cout << "YES\n";
#define pm cout << "-1\n";
#define pn cout << "NO\n";
#define pimp cout << "IMPOSSIBLE\n";
#define vr(v) v.begin (), v.end ()
#define rv(v) v.end (), v.begin ()
#define csort(nums) sort (nums.begin (), nums.end ());
#define sum(v) accumulate (v.begin (), v.end (), 0LL);
#define int long long
#define print(x) cout << x << endl ;

// Typedef
typedef vector<ll> vll;
typedef unordered_map<ll, ll, custom_hash> safehash;
typedef vector<vector<int>> mat;

// Utility functions
template <typename T>
void
printvec (vector<T> v)
{
  ll n = v.size ();
  fl (i, n) cout << v[i] << " ";
  cout << "\n";
}
template <typename T>
ll
sumvec (vector<T> v)
{
  ll n = v.size ();
  ll s = 0;
  fl (i, n) s += v[i];
  return s;
}

// Mathematical functions
ll
gcd (ll a, ll b)
{
  if (b == 0)
    return a;
  return gcd (b, a % b);
} //__gcd
ll
lcm (ll a, ll b)
{
  return (a / gcd (a, b) * b);
}
ll
moduloMultiplication (ll a, ll b, ll mod)
{
  ll res = 0;
  a %= mod;
  while (b)
    {
      if (b & 1)
        res = (res + a) % mod;
      b >>= 1;
    }
  return res;
}
ll
powermod (ll x, ll y, ll p)
{
  ll res = 1;
  x = x % p;
  if (x == 0)
    return 0;
  while (y > 0)
    {
      if (y & 1)
        res = (res * x) % p;
      y = y >> 1;
      x = (x * x) % p;
    }
  return res;
}
// Check
bool
isPrime (ll n)
{
  if (n <= 1)
    return false;
  if (n <= 3)
    return true;
  if (n % 2 == 0 || n % 3 == 0)
    return false;
  for (int i = 5; i * i <= n; i = i + 6)
    if (n % i == 0 || n % (i + 2) == 0)
      return false;
  return true;
}
void solve() {
    int l, r;
    cin >> l >> r;

    // Find the most significant bit (MSB) of r
    int msb_r = 31;
    while ((r >> msb_r) == 0) msb_r--;

    int no1 = 0, no2 = 0, no3 = 0;

    // Update no1 and no2 based on the constraints
    for (int i = 0; i <= 30; i++) {
        int temp = (no1 | (1 << i));
        if (temp <= r) 
            no1 = temp;
        else if ((no2 | (1 << i)) <= r) 
            no2 |= (1 << i);
    }

    // If no2 is less than l, modify it using MSB approach
    if (no2 < l) {
        for (int i = msb_r; i >= 0; i--) {
            int temp1 = (no2 | (1 << i));
            int temp2 = (no2 & ~(1LL << i));
            if (no2 < l) 
                no2 = temp1;
            else if (no2 > r) 
                no2 = temp2;
            else 
                break;
        }
    }

    // Update no3 based on no1 and no2
    for (int i = 0; i <= msb_r; i++) {
        bool bit1 = (no1 & (1 << i));
        bool bit2 = (no2 & (1 << i));
        int setBit = (no3 | (1 << i));
        if (bit1 == 0 && bit2 == 0) 
            no3 = setBit;
    }

    // If no3 is less than l, modify it using MSB approach
    if (no3 < l || no3 > r) 
    {

    for (int i = 0; i <= msb_r; i++) {
        bool bit1 = (no1 & (1 << i));
        bool bit2 = (no2 & (1 << i));
        int setBit = (no3 | (1 << i));
        if (bit1 == 0 || bit2 == 0) 
            no3 = setBit;
    }
        if (no3 < l || no3 > r) 
        {
        for (int i = msb_r; i >= 0; i--) {
            bool bit1 = (no1 & (1 << i));
            bool bit2 = (no2 & (1 << i));
            int setBit = (no3 | (1 << i));
            int unsetBit = (no3 & ~(1 << i));

            if (no3 < l) 
                no3 = setBit;
            else if (no3 > r) 
                no3 = unsetBit;
            else 
                break;
        }
        }
    }
    if (no3 == no1 || no3 == no2) 
    {
        while(no3 == no2 || no3 == no1) no3-- ;
        while (no3 < l || no3 == no2 || no3 == no1)  no3++ ;
    }
    cout << no1 << " " << no2 << " " << no3 << endl;
}
int32_t
main ()
{
  ios_base::sync_with_stdio (false);
  cin.tie (NULL);

  int t = 1;
  cin >> t ;

  while (t--)
    {
      solve ();
    }

  return 0;
}

