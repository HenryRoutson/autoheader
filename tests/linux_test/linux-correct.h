// This file was automatically created,
// any defitions, including typedefs, structs or #define
// have been moved to a -defs.h file of the same name

#include "linux-defs.h"

static void *crypto842_alloc_ctx(struct crypto_scomp *tfm);
static int crypto842_init(struct crypto_tfm *tfm);
static void crypto842_free_ctx(struct crypto_scomp *tfm, void *ctx);
static void crypto842_exit(struct crypto_tfm *tfm);
static int crypto842_compress(struct crypto_tfm *tfm,
			      const u8 *src, unsigned int slen,
			      u8 *dst, unsigned int *dlen);
static int crypto842_scompress(struct crypto_scomp *tfm,
			       const u8 *src, unsigned int slen,
			       u8 *dst, unsigned int *dlen, void *ctx);
static int crypto842_decompress(struct crypto_tfm *tfm,
				const u8 *src, unsigned int slen,
				u8 *dst, unsigned int *dlen);
static int crypto842_sdecompress(struct crypto_scomp *tfm,
				 const u8 *src, unsigned int slen,
				 u8 *dst, unsigned int *dlen, void *ctx);
