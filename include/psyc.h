#include <stdint.h>
#include <string.h>

#define PSYC_EPOCH      1440444041      // 2015-08-24 21:20:41 CET (Monday)

/** @brief Checks if short keyword string matches long keyword string
 */
int PSYC_matches(uint8_t* sho, unsigned int slen,
		 uint8_t* lon, unsigned int llen);

/** @brief Callback for PSYC_text() that produces a value for a match
 *
 * The application looks up a match such as _fruit from [_fruit] and
 * if found writes its current value from its variable store into the
 * outgoing buffer.. "Apple" for example. The template returns the
 * number of bytes written. 0 is a legal return value. Should the
 * callback return -1, PSYC_text leaves the original template text as is.
 */
typedef int (*PSYC_textCB)(uint8_t* match, unsigned int mlen,
		           uint8_t** buffer, unsigned int* blen);

/** @brief Fills out text templates by asking a callback for content
 *
 * Copies the contents of the template into the buffer while looking
 * for braceOpen and braceClose strings and calling the callback for
 * each enclosed string between these braces. Should the callback
 * return -1, the original template text is copied as is.
 *
 * By default PSYC's "[" and "]" are used but you can provide any other
 * brace strings such as "${" and "}" or "<!--" and "-->".
 *
 * See also http://about.psyc.eu/psyctext
 */
int PSYC_text(uint8_t* template, unsigned int tlen,
	      uint8_t** buffer, unsigned int* blen,
	      PSYC_textCB lookupValue,
	      char* braceOpen, char* braceClose);

