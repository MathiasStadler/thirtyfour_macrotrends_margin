function() {/*

 Copyright The Closure Library Authors.
 SPDX-License-Identifier: Apache-2.0
*/
    var f = this || self, h = function (a) { return a }; var n = function (a, b) { this.h = a === l && b || ""; this.g = m }, p = function (a) { return a instanceof n && a.constructor === n && a.g === m ? a.h : "type_error:Const" }, m = {}, l = {}; var r = void 0;/*

 SPDX-License-Identifier: Apache-2.0
*/
    var t, aa = function () {
        if (void 0 === t) {
            var a = null, b = f.trustedTypes; if (b && b.createPolicy) {
                try {
                    a = b.createPolicy("goog#html", { createHTML: h, createScript: h, createScriptURL: h })
                }
                catch (c) {
                    f.console && f.console.error(c.message)
                } t = a
            }
            else t = a
        }
        return t
    };
    var ca = function (a) { this.g = ba === ba ? a : "" };
    ca.prototype.toString = function () { return this.g + "" };
    var ba = {}, da = function (a) {
        var b = aa(); a = b ? b.createScriptURL(a) : a;
        return new ca(a)
    }; var ea = {}, u = function (a, b) { this.g = b === ea ? a : "" };
    u.prototype.toString = function () { return this.g.toString() };
    var ha = function () {
        var a = v, b = { message: fa(v) };
        var c = void 0 === c ? {} : c; this.error = a;
        this.context = b.context; this.msg = b.message || "";
        this.id = b.id || "jserror"; this.meta = c
    };
    var w = function (a) { w[" "](a); return a };
    w[" "] = function () { };
    var ia = RegExp("^(?:([^:/?#.]+):)?(?://(?:([^\\\\/?#]*)@)?([^\\\\/?#]*?)(?::([0-9]+))?(?=[\\\\/?#]|$))?([^?#]+)?(?:\\?([^#]*))?(?:#([\\s\\S]*))?$");
    var ja = function (a, b) { if (a) for (var c in a) Object.prototype.hasOwnProperty.call(a, c) && b(a[c], c, a) };
    var ka = RegExp("^https?://(\\w|-)+\\.cdn\\.ampproject\\.(net|org)(\\?|/|$)"), ma = function () { var a = la; this.g = x; this.h = a }, na = function (a, b) {
        this.url = a;
        this.j = !!b; this.depth = null
    };
    function oa(a) {
        f.google_image_requests || (f.google_image_requests = []);
        var b = f.document;
        b = void 0 === b ? document : b;
        b = b.createElement("img"); b.src = a; f.google_image_requests.push(b)
    };
    var y = function () { this.i = "&"; this.h = {}; this.o = 0; this.g = [] }, z = function (a, b) {
        var c = {};
        c[a] = b; return [c]
    }, qa = function (a, b, c, d, e) {
        var k = [];
        ja(a, function (g, A) { (g = pa(g, b, c, d, e)) && k.push(A + "=" + g) });
        return k.join(b)
    }, pa = function (a, b, c, d, e) {
        if (null == a) return ""; b = b || "&";
        c = c || ",$"; "string" == typeof c && (c = c.split(""));
        if (a instanceof Array) {
            if (d = d || 0, d < c.length) {
                for (var k = [], g = 0; g < a.length; g++)k.push(pa(a[g], b, c, d + 1, e));
                return k.join(c[d])
            }
        } else if ("object" == typeof a) return e = e || 0, 2 > e ? encodeURIComponent(qa(a, b, c, d, e + 1)) : "...";
        return encodeURIComponent(String(a))
    }, sa = function (a) {
        var b = "https://pagead2.googlesyndication.com/pagead/gen_204?id=jserror&", c = ra(a) - 27;
         if (0 > c) return "";
        a.g.sort(function (za, Aa) { return za - Aa });
         for (var d = null, e = "", k = 0; k < a.g.length; k++)for (var g = a.g[k], A = a.h[g], O = 0;
            O < A.length; O++) { if (!c) { d = null == d ? g : d;
                 break } var q = qa(A[O], a.i, ",$"); 
                 if (q) { q = e + q; 
                    if (c >= q.length) { c -= q.length; 
                        b += q; 
                        e = a.i; break } d = null == d ? g : d } } a = "";
        null != d && (a = e + "trn=" + d);
         return b + a
    }, ra = function (a) { var b = 1, c; 
        for (c in a.h) b = c.length > b ? c.length : b; return 3997 - b - a.i.length - 1 };
         var ta = function (a) { if (.01 > Math.random()) try { if (a instanceof y) var b = a;
             else b = new y, ja(a, function (d, e) { var k = b, g = k.o++;
                 d = z(e, d);
                  k.g.push(g); 
                  k.h[g] = d }); 
                  var c = sa(b); 
                  c && oa(c) } catch (d) { } };
    var fa = function (a) { var b = a.toString(); 
        a.name && -1 == b.indexOf(a.name) && (b += ": " + a.name); 
        a.message && -1 == b.indexOf(a.message) && (b += ": " + a.message); 
        if (a.stack) { a = a.stack; var c = b; 
            try { -1 == a.indexOf(c) && (a = c + "\n" + a); 
                for (var d; 
                    a != d;)d = a, a = a.replace(RegExp("((https?:/..*/)[^/:]*:\\d+(?:.|\n)*)\\2"), "$1"); b = a.replace(RegExp("\n *", "g"), "\n") } catch (e) { b = c } } return b }; ({})[3] = da(p(new n(l, "https://s0.2mdn.net/ads/richmedia/studio/mu/templates/hifi/hifi.js"))); ({})[3] = da(p(new n(l, "https://s0.2mdn.net/ads/richmedia/studio_canary/mu/templates/hifi/hifi_canary.js"))); var ua = /^([^;]+);(\d+);([\s\S]*)$/; var va = /^([a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?\.safeframe\.googlesyndication\.com|tpc\.googlesyndication\.com|secureframe\.doubleclick\.net|[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?\.safeframe\.usercontent\.goog)$/, wa = /^(pagead2\.googlesyndication\.com|googleads\.g\.doubleclick\.net)$/; var xa = function (a) { return function (b) { var c = a.hostname, d = va.test(c) || wa.test(c); c = [c]; var e = r; r = void 0; if (!d) { if (e) throw Error(e()); if (c && 0 < c.length) throw Error("[" + c.map(String).join(",") + "]"); throw Error(String(d)); } b = (d = aa()) ? d.createHTML(b) : b; return new u(b, ea) } }(location); if (window.name) try { var B, ya = window.name, C = ua.exec(ya); if (null === C) throw Error("Cannot parse serialized data. " + ya.substring(0, 50)); var D = +C[2], E = C[3]; if (D > E.length) throw Error("Parsed content size doesn't match. " + D + ":" + E.length); B = { m: C[1], content: E.substr(0, D), l: E.substr(D) }; var F = JSON.parse(B.l); window.name = ""; var Ba = B.content; F.goog_safeframe_hlt && (f.goog_safeframe_hlt = F.goog_safeframe_hlt); F._context && (f.AMP_CONTEXT_DATA = F._context); f.sf_ = { v: B.m, cfg: F }; document.open("text/html", "replace"); var G = xa(Ba); document.write(G instanceof u && G.constructor === u ? G.g : "type_error:SafeHtml"); document.close(); f.sf_ && (window.name = "") } catch (a) { var v = a, H; try { var I = new y; I.g.push(1); I.h[1] = z("context", 507); v.error && v.meta && v.id || (v = new ha); if (v.msg) { var Ca = v.msg.substring(0, 512); I.g.push(2); I.h[2] = z("msg", Ca) } var Da = [v.meta || {}]; I.g.push(3); I.h[3] = Da; var J = f, K = [], L, M = null, N; do { N = J; var P; try { var Q; if (Q = !!N && null != N.location.href) b: { try { w(N.foo); Q = !0; break b } catch (b) { } Q = !1 } P = Q } catch (b) { P = !1 } P ? (L = N.location.href, M = N.document && N.document.referrer || null) : (L = M, M = null); K.push(new na(L || "")); try { J = N.parent } catch (b) { J = null } } while (J && N != J); for (var R = 0, Ea = K.length - 1; R <= Ea; ++R)K[R].depth = Ea - R; N = f; if (N.location && N.location.ancestorOrigins && N.location.ancestorOrigins.length == K.length - 1) for (var S = 1; S < K.length; ++S) { var T = K[S]; T.url || (T.url = N.location.ancestorOrigins[S - 1] || "", T.j = !0) } for (var x = new na(f.location.href, !1), Fa = null, U = K.length - 1, V = U; 0 <= V; --V) { var W = K[V]; !Fa && ka.test(W.url) && (Fa = W); if (W.url && !W.j) { x = W; break } } var la = null, Ga = K.length && K[U].url; 0 != x.depth && Ga && (la = K[U]); H = new ma; if (H.h) { var Ha = H.h.url || ""; I.g.push(4); I.h[4] = z("top", Ha) } var Ia = { url: H.g.url || "" }, X; if (H.g.url) { var Y; Y = H.g.url.match(ia); var Ja = Y[1], Ka = Y[3], La = Y[4], Z = ""; Ja && (Z += Ja + ":"); Ka && (Z += "//", Z += Ka, La && (Z += ":" + La)); X = Z } else X = ""; var Ma = [Ia, { url: X }]; I.g.push(5); I.h[5] = Ma; ta(I) } catch (b) { try { ta({ context: "ecmserr", rctx: 507, msg: fa(b), url: H && H.g.url }) } catch (c) { } } };
}).call(this);
