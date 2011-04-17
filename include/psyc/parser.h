#include <stdint.h>
#include <string.h>

enum PSYC_Flags
{
	PSYC_HEADER_ONLY = 1
};


enum PSYC_ReturnCodes
{
	PSYC_METHOD = 1,
	PSYC_INSUFFICIENT,
	PSYC_ROUTING,
	PSYC_ENTITY,
	PSYC_COMPLETE,
	PSYC_HEADER_COMPLETE, 
};


typedef struct 
{
	unsigned int length;
	const uint8_t * ptr;
} PSYC_Array;

typedef struct 
{
	unsigned int cursor;           // current position in buffer
	PSYC_Array buffer;
	uint8_t flags;

	char inBody;
	unsigned int length;
	unsigned int contentLength;
} PSYC_State;

#ifndef PSYC_COMPILE_LIBRARY
/* @brief shortcut for creating an array 
 *
 * @param memory pointer to the buffer
 * @param length length of that buffer
 *
 * @returns an instance of the PSYC_Array struct */
inline PSYC_Array PSYC_CreateArray (uint8_t* const memory, unsigned int length)
{
	PSYC_Array arr = {length, memory};

	return arr;
}
/* @brief initiates the state struct with flags 
 *
 * @param state pointer to the state struct that should be initiated
 * @param flags the flags that one ones to set, se PSYC_Flags */
inline void PSYC_initState2 (PSYC_State* state, uint8_t flags )
{
	memset(state, 0, sizeof(PSYC_State));
	state->flags = flags;
}

inline void PSYC_initState (PSYC_State* state)
{
	memset(state, 0, sizeof(PSYC_State));
}

inline void PSYC_nextBuffer (PSYC_State* state, PSYC_Array newBuf)
{
	state->buffer = newBuf; 
}

inline unsigned int PSYC_getBodyLength (PSYC_State* state)
{
	return state->length;
}
#endif

int PSYC_parse(PSYC_State* state, 
               PSYC_Array* name, PSYC_Array* value, 
               uint8_t* modifier, unsigned long *expectedBytes);


