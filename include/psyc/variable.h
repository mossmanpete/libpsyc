/**
 * @file psyc/variable.h
 */

#ifndef PSYC_VARIABLE_H
#define PSYC_VARIABLE_H

#include "packet.h"

/// Routing variables in alphabetical order.
extern const PsycString psyc_routing_vars[];

// Variable types in alphabetical order.
extern const PsycDictInt psyc_var_types[];

/// Method names in alphabetical order.
extern const PsycDictInt psyc_methods[];

extern const size_t psyc_routing_vars_num;
extern const size_t psyc_var_types_num;
extern const size_t psyc_methods_num;

/**
 * Is this a routing variable name?
 */
PsycBool
psyc_var_is_routing (const char *name, size_t len);

/**
 * Get the type of variable name.
 */
PsycType
psyc_var_type (const char *name, size_t len);

/**
 * Is this a list variable name?
 */
static inline PsycBool
psyc_var_is_list (const char *name, size_t len)
{
    return len < 5 || memcmp(name, "_list", 5) != 0 || (len > 5 && name[5] != '_')
	? PSYC_FALSE : PSYC_TRUE;
}

PsycMethod
psyc_method (char *method, size_t methodlen, PsycMethod *family, unsigned int *flag);

#endif
