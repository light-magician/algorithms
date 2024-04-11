class Solution:
    def strStr(self, haystack: str, needle: str) -> int:
        '''
        so what i keep missing is that i have to keep reprocessing 
        from where it all started
        '''
        # edge cases
        if needle == '':
            return 0
        if len(haystack) < len(needle):
            return -1

        # haystack and needle character index
        hi, ni = 0, 0
        # should we increment the hi char we are comparing
        should_increment = True
        while hi < len(haystack):
            if haystack[hi] == needle[ni]:
                ni += 1
                if ni == len(needle):
                    # the start of that string
                    return hi - (len(needle) - 1)
            else:
                '''
                on resets we want to try that same hi char again
                but reset the ni char so we set our flag false
                but we only want to flip should increment in a reprocess case
                '''
                if ni > 0: # indicator of a reprocess case
                    '''
                    if we get reset we need to send this back to 
                    where the match started + 1
                    '''
                    hi = (hi - ni) + 1 # where match started + 1
                    should_increment = False
                ni = 0
                
            if not should_increment:
                should_increment = True
            else:
                hi += 1
        return -1