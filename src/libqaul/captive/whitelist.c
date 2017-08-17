/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "../qaullib_private.h"
#include "../qaullib_threads.h"
#include "whitelist.h"
#include "whitelist_LL.h"


// ------------------------------------------------------------
void ql_whitelist_add (union olsr_ip_addr *ip)
{
	struct qaul_whitelist_LL_item *item;

	// check if entry exists
	if(Qaullib_Whitelist_LL_Find_ByIP(ip, &item))
	{
		// update timestamp
		item->time = time(NULL);
	}
	else
	{
		// add list entry
		Qaullib_Whitelist_LL_Add(ip);
	}
}


// ------------------------------------------------------------
int ql_whitelist_check (union olsr_ip_addr *ip)
{
	struct qaul_whitelist_LL_item *item;

	return Qaullib_Whitelist_LL_Find_ByIP(ip, &item);
}

// ------------------------------------------------------------
void Qaullib_Whitelist_LL_Init (void)
{
	qaul_whitelist_LL_first = 0;
}

// ------------------------------------------------------------
int Qaullib_Whitelist_LL_NextItem (struct qaul_whitelist_LL_item *item)
{
	if(item != 0 && item->next != 0)
	{
		item = item->next;
		return 1;
	}
	return 0;
}

// ------------------------------------------------------------
int Qaullib_Whitelist_LL_PrevItem (struct qaul_whitelist_LL_item *item)
{
	if(
			item != 0 &&
			item != qaul_whitelist_LL_first &&
			item->prev != 0
			)
	{
		item = item->prev;
		return 1;
	}
	return 0;
}

// ------------------------------------------------------------
void Qaullib_Whitelist_LL_Add (union olsr_ip_addr *ip)
{
	struct qaul_whitelist_LL_item *new_item;
	new_item = (struct qaul_whitelist_LL_item *)malloc(sizeof(struct qaul_whitelist_LL_item));

	if(QAUL_DEBUG)
		printf("Qaullib_Whitelist_LL_Add\n");

	// fill in content
	memcpy((char *)&new_item->ip, ip, sizeof(union olsr_ip_addr));
	new_item->time = time(NULL);

	// lock
	pthread_mutex_lock( &qaullib_mutex_whitelistLL );

	// create links
	new_item->next = qaul_whitelist_LL_first;
	qaul_whitelist_LL_first = new_item;

	// unlock
	pthread_mutex_unlock( &qaullib_mutex_whitelistLL );
}

// ------------------------------------------------------------
int Qaullib_Whitelist_LL_Find_ByIP (union olsr_ip_addr *ip, struct qaul_whitelist_LL_item **item)
{
	struct qaul_whitelist_LL_item *myitem = qaul_whitelist_LL_first;

	while(Qaullib_Whitelist_LL_NextItem(myitem))
	{
		// check if older than timeout
		if(myitem->time < time(NULL) -CAPTIVE_WHITELIST_TIMEOUT)
		{
			Qaullib_Whitelist_LL_Delete(myitem);
		}
		else
		{
			// compare IP
			if(memcmp(&myitem->ip, ip, qaul_ip_size) == 0)
			{
				*item = myitem;
				return 1;
			}
		}
	}

	return 0;
}

// ------------------------------------------------------------
void Qaullib_Whitelist_LL_Delete (struct qaul_whitelist_LL_item *item)
{
	// lock
	pthread_mutex_lock( &qaullib_mutex_whitelistLL );

	if(item->prev != 0)
		item->prev->next = item->next;
	if(item->next != 0)
		item->next->prev = item->prev;

	// unlock
	pthread_mutex_unlock( &qaullib_mutex_whitelistLL );

	free(item);
}

// ------------------------------------------------------------
void Qaullib_Whitelist_LL_Clean (void)
{
	struct qaul_whitelist_LL_item *item;

	item = qaul_whitelist_LL_first;

	// which is older than the timeout
	while(Qaullib_Whitelist_LL_NextItem(item))
	{
		if(item->time < time(NULL) -CAPTIVE_WHITELIST_TIMEOUT)
		{
			Qaullib_Whitelist_LL_Delete(item);
		}
	}
}

